use std::{
    fs::{self, File},
    io::{self, Error, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use reqwest::{Client, header};
use tokio::task;

/// 下载配置
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub url: String,
    pub output_path: PathBuf,
    pub temp_dir: PathBuf,
    pub max_retries: usize,
    pub timeout: Duration,
    pub max_threads: usize,
}

/// 下载进度
#[derive(Debug, Clone, Default)]
pub struct DownloadProgress {
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub speed: f64,
}

impl DownloadProgress {
    pub fn is_finished(&self) -> bool{
        self.total_bytes == self.downloaded_bytes
    }
}

/// 下载器
pub struct Downloader {
    client: Client,
    config: DownloadConfig,
    progress: Arc<Mutex<DownloadProgress>>,
}

impl Downloader {
    pub fn new(config: DownloadConfig) -> io::Result<Self> {
        // 创建临时目录
        fs::create_dir_all(&config.temp_dir)?;

        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self {
            client,
            config,
            progress: Arc::new(Mutex::new(DownloadProgress::default())),
        })
    }

    /// 启动下载
    pub async fn start(&self) -> io::Result<()> {
        let response = self
            .client
            .head(&self.config.url)
            .send()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // 检查是否支持断点续传
        let supports_resume = response
            .headers()
            .get(header::ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            == Some("bytes");

        let total_size = response.content_length().unwrap_or(0);
        self.progress.lock().unwrap().total_bytes = total_size;

        if supports_resume && total_size > 0 && self.config.max_threads > 1 {
            self.download_concurrent().await
        } else {
            self.download_single().await
        }
    }

    /// 单线程下载
    async fn download_single(&self) -> io::Result<()> {
        let response = self
            .client
            .get(&self.config.url)
            .send()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !response.status().is_success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to download file",
            ));
        }

        let total_size = response.content_length().unwrap_or(0);
        self.progress.lock().unwrap().total_bytes = total_size;

        let mut file = File::create(&self.config.output_path)?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        file.write_all(&bytes)?;

        let mut prog = self.progress.lock().unwrap();
        prog.downloaded_bytes = total_size;

        Ok(())
    }

    /// 多线程并发下载
    async fn download_concurrent(&self) -> io::Result<()> {
        let total_size = self.progress.lock().unwrap().total_bytes;
        let chunk_size = total_size / self.config.max_threads as u64;

        let mut handles = vec![];

        for i in 0..self.config.max_threads {
            let start = i as u64 * chunk_size;
            let end = if i == self.config.max_threads - 1 {
                total_size - 1
            } else {
                (i + 1) as u64 * chunk_size - 1
            };

            let config = self.config.clone();
            let client = self.client.clone();
            let progress = self.progress.clone();

            let handle = task::spawn(async move {
                for retry in 0..=config.max_retries {
                    let temp_file = config.temp_dir.join(format!("part_{}", i));
                    let downloaded = fs::metadata(&temp_file).map(|m| m.len()).unwrap_or(0);

                    if downloaded >= end - start + 1 {
                        break;
                    }

                    let result = Downloader::download_chunk(
                        &client,
                        &config.url,
                        start + downloaded,
                        end,
                        &temp_file,
                        progress.clone(),
                    )
                    .await;

                    if result.is_ok() || retry == config.max_retries {
                        return result;
                    }
                }
                Ok(())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))??;
        }

        self.merge_files().await
    }

    /// 下载文件块
    async fn download_chunk(
        client: &Client,
        url: &str,
        start: u64,
        end: u64,
        temp_path: &Path,
        progress: Arc<Mutex<DownloadProgress>>,
    ) -> io::Result<()> {
        let mut file = File::options().create(true).append(true).open(temp_path)?;
        let mut response = client
            .get(url)
            .header(header::RANGE, format!("bytes={}-{}", start, end))
            .send()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        {
            file.write_all(&chunk)?;

            let mut prog = progress.lock().unwrap();
            prog.downloaded_bytes += chunk.len() as u64;
        }

        Ok(())
    }

    /// 合并临时文件
    async fn merge_files(&self) -> io::Result<()> {
        let mut output = File::create(&self.config.output_path)?;

        for i in 0..self.config.max_threads {
            let mut part_file = File::open(self.config.temp_dir.join(format!("part_{}", i)))?;
            io::copy(&mut part_file, &mut output)?;
        }

        // 清理临时文件
        fs::remove_dir_all(&self.config.temp_dir)?;
        Ok(())
    }

    /// 获取下载进度
    pub fn get_progress(&self) -> DownloadProgress {
        self.progress.lock().unwrap().clone()
    }
}

pub struct DownloadManager {
    downloaders: Vec<Arc<Downloader>>,
}

pub struct DownloadManagerConfig {
    pub url: String,
    pub dest: PathBuf,
}

impl DownloadManager {
    pub fn new(configs: &Vec<DownloadManagerConfig>) -> Result<Self, Error> {
        let mut dls: Vec<Arc<Downloader>> = Vec::new();
        for i in configs.iter() {
            let dl = Downloader::new(DownloadConfig {
                url: i.url.to_string(),
                output_path: i.dest.to_path_buf(),
                temp_dir: dirs_next::cache_dir()
                    .unwrap()
                    .join("PCL-Nova")
                    .join("cache")
                    .join("download"),
                max_retries: 3,
                timeout: Duration::from_millis(30_000),
                max_threads: 2,
            });
            match dl {
                Ok(e) => dls.push(Arc::new(e)),
                Err(e) => return Err(e),
            }
        }
        Ok(Self { downloaders: dls })
    }

    pub fn start(&self){
        for downloader in self.downloaders.iter() {
            let downloader = std::sync::Arc::clone(downloader);
            tokio::spawn(async move {
                downloader.start().await;
            });
        }
    }

    pub fn wait_for_end(&self){
        loop {
            let mut _all_finished: bool = true;
            for i in self.downloaders.iter(){
                let i = std::sync::Arc::clone(i);
                _all_finished = _all_finished && i.get_progress().is_finished();
            }
            if _all_finished {
                break;
            }
        }
    }
}
