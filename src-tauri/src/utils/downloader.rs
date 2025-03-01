use std::sync::{Arc, Mutex};
use tokio::{io::AsyncWriteExt, task::JoinHandle};

use crate::core::NovaError;

use futures_util::stream::StreamExt;
use std::{path::PathBuf, str::FromStr, sync::atomic::AtomicU64};

use uuid::Uuid;

// 字符串切割适配多平台
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

type Progresser = Box<dyn Fn(Option<u64>, Option<u64>, Option<u64>) -> bool>;

pub struct Downloader {
    url: String,
    dest: PathBuf,
    concurrency: u64,
    client: reqwest::Client,
    total_bytes: u64,
    downloaded_bytes: AtomicU64,
    max_retries: u16,
    pub terminated: bool,
    progresser: Mutex<Progresser>,
}

impl Downloader {
    pub fn new<S: ToString + ?Sized, F>(
        url: &S,
        dest: &S,
        mut concurrency: u64,
        retries: u16,
        timeout: u16,
        progresser: Option<F>,
    ) -> Self
    where
        F: Fn(Option<u64>, Option<u64>, Option<u64>) -> bool + 'static,
    {
        let tokio_runtime =
            tokio::runtime::Runtime::new().expect("Failed to create tokio runtime.");
        let client = reqwest::Client::builder()
            .read_timeout(std::time::Duration::from_millis(timeout as u64))
            //.default_headers(headers)
            .build()
            .expect("Failed to build network client.");
        let mut total_bytes = tokio_runtime.block_on(async {
            let response = client
                .head(url.to_string().as_str())
                .send()
                .await
                .expect("Failed to get header.");
            response.content_length()
        });
        if total_bytes.is_none() {
            concurrency = 1;
            total_bytes = Some(0);
        }
        let default_progresser = |_: Option<u64>, _: Option<u64>, _: Option<u64>| false;
        Self {
            url: url.to_string().clone(),
            dest: PathBuf::from_str(dest.to_string().as_str())
                .expect("Failed to create PathBuf from String."),
            concurrency,
            client,
            total_bytes: total_bytes.unwrap(),
            downloaded_bytes: AtomicU64::new(0),
            max_retries: retries,
            terminated: false,
            progresser: match progresser {
                Some(p) => Mutex::new(Box::new(p)),
                None => Mutex::new(Box::new(default_progresser)),
            },
        }
    }

    /// Start whole downloader process.
    pub fn start(&mut self) -> Result<(), NovaError> {
        todo!()
    }

    async fn download_single_thread(&mut self) -> Result<(), NovaError> {
        let path = self.dest.to_str().unwrap().to_string();
        let mut last_slash = 0;
        for (index, &c) in path.as_bytes().iter().enumerate() {
            if c == b'/' {
                last_slash = index;
            }
        }
        std::fs::create_dir_all(&path[0..last_slash]).expect("Failed to create directory.");
        let mut file = tokio::fs::File::create(path)
            .await
            .expect("Failed to create file.");
        for _retry in 1..self.max_retries {
            let response = self
                .client
                .get(&self.url)
                .send()
                .await
                .expect("Bad request.");
            if response.status().as_u16() < 400 {
                let mut stream = response.bytes_stream();
                while let Some(buffer) = stream.next().await {
                    // it should work
                    let buffer_ = &buffer.expect("Failed to read buffer.");
                    file.write_all(&buffer_)
                        .await
                        .expect("等 nova err 实现好点再说");
                    self.downloaded_bytes
                        .fetch_add(buffer_.len() as u64, std::sync::atomic::Ordering::SeqCst);
                    if let Ok(p) = self.progresser.lock() {
                        p(
                            Some(
                                self.downloaded_bytes
                                    .load(std::sync::atomic::Ordering::SeqCst),
                            ),
                            Some(self.total_bytes),
                            Some(buffer_.len() as u64),
                        );
                    }
                    
                }
                return Ok(());
            }
        }
        Err(NovaError::msg("Bad requests."))
    }

    async fn download_multi_thread(&self, concurrency: usize) -> Result<(), NovaError> {
        // 创建目标目录和文件（异步版本）
        let path = self.dest.to_str()
            .ok_or_else(|| NovaError::msg("Invalid destination path"))?;
        
        // 异步创建目录
        if let Some(parent) = self.dest.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| NovaError::msg("Failed to create directory"))?;
        }
        
        // 异步创建文件
        tokio::fs::File::create(&self.dest)
            .await
            .map_err(|e| NovaError::msg("Failed to create file"))?;
    
        // 准备缓存目录
        let cache_dir = dirs_next::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("PCL-Nova")
            .join("cache")
            .join("downloads")
            .join(Uuid::new_v4().to_string());
        
        tokio::fs::create_dir_all(&cache_dir)
            .await
            .map_err(|e| NovaError::msg("Failed to create cache directory"))?;
    
        let chunk_size = self.total_bytes / concurrency as u64;
        let mut handles = vec![];
    
        // 使用Arc包装self（需要结构体实现Sync+Send）
        let self_arc = Arc::new(self);
        let shared_cache_dir = Arc::new(cache_dir);
    
        for i in 0..concurrency {
            let self_clone = Arc::clone(&self_arc);
            let cache_dir = Arc::clone(&shared_cache_dir);
            
            let start = i as u64 * chunk_size;
            let end = if i == concurrency - 1 {
                self.total_bytes - 1
            } else {
                (i + 1) as u64 * chunk_size - 1
            };
    
            let handle = tokio::task::spawn(async move {
                let temp_file = cache_dir.join(format!("part_{}", i));
    
                // 异步获取文件元数据
                let downloaded = match tokio::fs::metadata(&temp_file).await {
                    Ok(meta) => meta.len(),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
                    Err(e) => return Err(NovaError::msg(&e.to_string())),
                };
    
                // 验证下载范围
                let expected_size = end - start + 1;
                if downloaded > expected_size {
                    return Err(NovaError::msg("Corrupted temporary file"));
                }
    
                // 执行分块下载
                // self_clone.download_chunk(
                //     start + downloaded,
                //     end,
                //     cache_dir
                // ).await?;
    
                Ok(())
            });
    
            handles.push(handle);
        }
    
        // 启动每个块线程
        for handle in handles {
            handle
                .await
                .map_err(|e| NovaError::msg("Bad thread"))??;
        }
    
        // 合并文件（需要确保self实现了Sync）
        //self.merge_files().await?;
    
        Ok(())
    }

    /// 下载文件块
    async fn download_chunk(
        &mut self,
        start: u64,
        end: u64,
        // &cache_path: Arc<PathBuf>, //我先注释一下
    ) -> Result<(), NovaError> {
        todo!();
        // let mut file = File::options().create(true).append(true).open(cache_path)?;
        // let mut response = client
        //     .get(url)
        //     .header(header::RANGE, format!("bytes={}-{}", start, end))
        //     .send()
        //     .await
        //     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // while let Some(chunk) = response
        //     .chunk()
        //     .await
        //     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        // {
        //     file.write_all(&chunk)?;

        //     let mut prog = progress.lock().unwrap();
        //     prog.downloaded_bytes += chunk.len() as u64;
        // }

        // Ok(())
    }

    async fn merge_files(&mut self) {
        todo!()
    }

    pub fn terminate(&mut self) {
        todo!()
    }
}
