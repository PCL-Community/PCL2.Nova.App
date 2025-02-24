use reqwest::header::HeaderValue;
use tokio::io::AsyncWriteExt;
use std::sync::Mutex;

use crate::core::minecraft::NovaError;

use super::net;
use std::{path::PathBuf, str::FromStr, sync::atomic::AtomicU64};
use futures_util::stream::StreamExt;

// 字符串切割适配多平台
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

type Progresser = Box<dyn Fn(Option<u64>, Option<u64>, Option<u64>) -> bool>;

pub struct Downloader{
    url: String,
    dest: PathBuf,
    concurrency: u64,
    client: reqwest::Client,
    total_bytes: u64,
    downloaded_bytes: AtomicU64,
    max_retries: u16,
    pub terminated: bool,
    progresser: Mutex<Progresser>
}

impl Downloader {
    pub fn new<S: ToString + ?Sized, F>(
        url: &S, 
        dest: &S,
        mut concurrency: u64,
        retries: u16,
        timeout: u16,
        progresser: Option<F>
    ) -> Self 
    where F: Fn(Option<u64>, Option<u64>, Option<u64>) -> bool + 'static
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
            let length = response
                .content_length();
            return length;
        });
        if total_bytes == None {
            concurrency = 1;
            total_bytes = Some(0);
        }
        let default_progresser = |_: Option<u64> ,_: Option<u64>, _: Option<u64> | false;
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
            }
        }
    }

    pub fn start(&mut self) -> Result<(), NovaError> {
        Ok(())
    }

    async fn download_single_thread(&mut self) -> Result<(), NovaError> {
        let path = self.dest.to_str().unwrap().to_string();
        let mut last_slash = 0;
        let mut index = 0;
        for &c in path.as_str().as_bytes() {
            if c == b'/' {
                last_slash = index;
            }
            index += 1;
        }
        std::fs::create_dir_all(path[0..last_slash].to_string())
            .expect("Failed to create directory.");
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
                let mut stream = response
                .bytes_stream();
                while let Some(buffer) = stream.next().await { // it should work
                    let buffer_ = &buffer.expect("Failed to read buffer.");
                    file.write(&buffer_).await
                        .expect("Failed to write data into file.");
                    self.downloaded_bytes.fetch_add(buffer_.len() as u64, std::sync::atomic::Ordering::SeqCst);
                    match self.progresser.lock() {
                        Ok(p) => {
                          p(Some(self.downloaded_bytes.load(std::sync::atomic::Ordering::SeqCst)), Some(self.total_bytes), Some(buffer_.len() as u64));  
                        },
                        Err(_) => (),
                    }
                }
                return Ok(());
            }
        }
        Err(NovaError::msg("Bad requests."))
    }

    async fn download_multi_thread(&mut self) -> Result<(), NovaError> {
        Ok(())
    }


}
