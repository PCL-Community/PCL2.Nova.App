use reqwest::header::HeaderValue;

use crate::core::minecraft::NovaError;

use super::net;
use std::{path::PathBuf, str::FromStr, sync::atomic::AtomicU64};


pub struct Downloader {
    url: String,
    dest: PathBuf,
    concurrency: u64,
    client: net::HttpClient,
    total_bytes: u64,
    downloaded_bytes: AtomicU64,
    download_speed: AtomicU64,
    pub terminated: bool
}

impl Downloader {
    pub fn new<S: ToString + ?Sized>(url: &S, dest: &S, mut concurrency: u64) -> Self {
        let tokio_runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime.");
        let client = net::HttpClient::new();
        let mut total_bytes = tokio_runtime.block_on(async {
            let response = client.head(url.to_string().as_str()).await
                .expect("Failed to get header.");
            let length: i64 = response.header.get(reqwest::header::CONTENT_LENGTH)
                .unwrap_or(&HeaderValue::from_str("-1").expect("Failed to create a void header."))
                .to_str()
                .expect("Failed to convert Content-Length header into String.")
                .parse()
                .expect("Failed to convert str to i64.");
            return length;
        });
        if total_bytes == -1 {
            concurrency = 1;
            total_bytes = 0;
        }
        Self {
            url: url.to_string().clone(),
            dest: PathBuf::from_str(dest.to_string().as_str()).expect("Failed to create PathBuf from String."),
            concurrency,
            client,
            total_bytes: total_bytes as u64,
            downloaded_bytes: AtomicU64::new(0),
            download_speed: AtomicU64::new(0),
            terminated: false
        }
    }

    pub fn start(&mut self) -> Result<(), NovaError> {
        Ok(())
    }

    fn download_single_thread(&mut self) -> Result<(), NovaError> {
        let mut path = self.dest.to_str().unwrap().to_string();
        let mut last_slash = 0;
        let mut index = 0;
        for c in unsafe { path.as_mut_vec().iter() } {
            if *c == b'/' {
                last_slash = index;
            }
            index += 1;
        }
        std::fs::create_dir_all(path[0..last_slash].to_string()).expect("Failed to create directory.");
        let filename = path[(last_slash + 1)..path.len()].to_string();
        
        Ok(())
    }
}