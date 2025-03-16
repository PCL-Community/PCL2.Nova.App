// use core::borrow;
// use std::{
//     path::PathBuf,
//     str::FromStr,
//     sync::{
//         atomic::{AtomicBool, AtomicU64, Ordering}, Arc
//     }, thread,
// };
// use tokio::{
//     fs::File,
//     io::{AsyncReadExt, AsyncWriteExt},
//     sync::Mutex,
//     task::JoinHandle,
// };

// use crate::NovaMCoreError;

// use futures_util::stream::StreamExt;
// use reqwest::header;
// use uuid::Uuid;

// // 字符串切割适配多平台
// #[cfg(unix)]
// use std::os::unix::ffi::OsStrExt;
// #[cfg(windows)]
// use std::os::windows::ffi::OsStrExt;

// type Progresser = Box<dyn Fn(Option<u64>, Option<u64>, Option<u64>) -> bool + Send + Sync>;

// pub struct Downloader {
//     url: String,
//     dest: PathBuf,
//     concurrency: u64,
//     client: Arc<reqwest::Client>,
//     total_bytes: Arc<AtomicU64>,
//     downloaded_bytes: Arc<AtomicU64>,
//     max_retries: u16,
//     timeout: Arc<AtomicU64>,
//     pub terminated: Arc<AtomicBool>,
//     progresser: Arc<tokio::sync::Mutex<Progresser>>,
// }

// pub struct DownloaderPointer {
//     pub pointer: *mut Downloader
// }

// impl DownloaderPointer {
//     fn from(value: &mut Downloader) -> Self {
//         Self {
//             pointer: core::ptr::addr_of_mut!(*value)
//         }
//     }
// }

// unsafe impl Send for DownloaderPointer {}

// unsafe impl Sync for DownloaderPointer {}

// impl Downloader {
//     pub fn new<S: ToString + ?Sized, F>(
//         url: &S,
//         dest: &S,
//         mut concurrency: u64,
//         retries: u16,
//         timeout: u16,
//         progresser: Option<F>,
//     ) -> Self
//     where
//         F: Fn(Option<u64>, Option<u64>, Option<u64>) -> bool + Send + Sync + 'static,
//     {
//         let tokio_runtime =
//             tokio::runtime::Runtime::new().expect("Failed to create tokio runtime.");
//         let client = reqwest::Client::builder()
//             .read_timeout(std::time::Duration::from_millis(timeout as u64))
//             //.default_headers(headers)
//             .build()
//             .expect("Failed to build network client.");
//         let mut total_bytes = tokio_runtime.block_on(async {
//             let response = client
//                 .head(url.to_string().as_str())
//                 .send()
//                 .await
//                 .expect("Failed to get header.");
//             response.content_length()
//         });
//         if total_bytes.is_none() {
//             concurrency = 1;
//             total_bytes = Some(0);
//         }
//         let default_progresser = |_: Option<u64>, _: Option<u64>, _: Option<u64>| false;
//         Self {
//             url: url.to_string().clone(),
//             dest: PathBuf::from_str(dest.to_string().as_str())
//                 .expect("Failed to create PathBuf from String."),
//             concurrency,
//             client: Arc::new(client),
//             total_bytes: Arc::new(AtomicU64::new(total_bytes.unwrap())),
//             timeout: Arc::new(AtomicU64::new(timeout as u64)),
//             downloaded_bytes: Arc::new(AtomicU64::new(0)),
//             max_retries: retries,
//             terminated: Arc::new(AtomicBool::from(false)),
//             progresser: Arc::new(Mutex::new(match progresser {
//                 Some(p) => Box::new(p) as Progresser,
//                 None => Box::new(default_progresser) as Progresser,
//             })),
//         }
//     }

//     /// Start whole downloader process.
//     pub async fn start(&'static mut self) -> Result<(), NovaMCoreError> {
//         if self.concurrency == 1 || self.total_bytes.load(Ordering::SeqCst) == 0 {
//             self.download_single_thread().await
//         } else {
//             self.download_multi_thread(self.concurrency as usize).await
//         }
//     }

//     async fn download_single_thread(&mut self) -> Result<(), NovaMCoreError> {
//         let path = self.dest.to_str().unwrap().to_string();
//         let mut last_slash = 0;
//         for (index, &c) in path.as_bytes().iter().enumerate() {
//             if c == b'/' {
//                 last_slash = index;
//             }
//         }
//         std::fs::create_dir_all(&path[0..last_slash]).expect("Failed to create directory.");
//         let mut file = tokio::fs::File::create(path)
//             .await
//             .expect("Failed to create file.");
//         for _retry in 1..self.max_retries {
//             let response = self
//                 .client
//                 .get(&self.url)
//                 .send()
//                 .await
//                 .expect("Bad request.");
//             if response.status().as_u16() < 400 {
//                 let mut stream = response.bytes_stream();
//                 while let Some(buffer) = stream.next().await {
//                     // it should work
//                     let buffer_ = &buffer.expect("Failed to read buffer.");
//                     file.write_all(&buffer_)
//                         .await
//                         .expect("等 nova err 实现好点再说");
//                     self.downloaded_bytes
//                         .fetch_add(buffer_.len() as u64, std::sync::atomic::Ordering::SeqCst);
//                     let p = self.progresser.lock().await;
//                     p(
//                         Some(
//                             self.downloaded_bytes
//                                 .load(std::sync::atomic::Ordering::SeqCst),
//                         ),
//                         Some(self.total_bytes.load(Ordering::SeqCst)),
//                         Some(buffer_.len() as u64),
//                     );
//                 }
//                 return Ok(());
//             }
//         }
//         Err(NovaMCoreError::msg("Bad requests."))
//     }

//     async fn download_multi_thread(&mut self, concurrency: usize) -> Result<(), NovaMCoreError> {
//         // 创建缓存目录
//         let cache_dir = dirs_next::cache_dir()
//             .unwrap_or_else(|| PathBuf::from(".cache"))
//             .join("Nova")
//             .join(Uuid::new_v4().to_string());

//         tokio::fs::create_dir_all(&cache_dir)
//             .await
//             .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;

//         // 创建目标文件
//         File::create(&self.dest)
//             .await
//             .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;

//         let total_bytes = self.total_bytes.load(Ordering::SeqCst);
//         let chunk_size = self.total_bytes.load(Ordering::SeqCst) / concurrency as u64;
//         let mut handles = Vec::with_capacity(concurrency);

//         // let download_chunk = async move |
//         //     pointer: Arc<Mutex<&mut Downloader>>,
//         //     url: String,
//         //     start: u64,
//         //     end: u64,
//         //     part_num: usize,
//         //     cache_dir: PathBuf,
//         // | -> Result<(), NovaMCoreError> {
            
//         // };


//         for i in 0..concurrency {
//             let start = i as u64 * chunk_size;
//             let end = if i == concurrency - 1 {
//                 self.total_bytes.load(Ordering::SeqCst) - 1
//             } else {
//                 (i + 1) as u64 * chunk_size - 1
//             };

//             let cache_dir: Arc<PathBuf> = Arc::new(cache_dir.clone());
//             let borrow = Arc::new(Mutex::new(self));
//             unsafe {
//                 // handles.push(thread::spawn(
//                 //     // async move || -> Result<(), NovaMCoreError> {
                        
//                 //     // }
//                 // ));
//             }
            
//         }

//         // 等待所有任务完成
//         let mut results = Vec::with_capacity(handles.len());
//         for handle in handles {
//             results.push(handle.await.map_err(|e| NovaMCoreError::msg(&e.to_string()))??);
//         }

//         // 合并文件
//         (&mut *self).merge_files(&cache_dir).await?;

//         // 清理缓存
//         /* tokio::fs::remove_dir_all(&cache_dir)
//         .await
//         .map_err(|e| NovaMCoreError::io_error(e))?; */

//         // Ok(());
//         todo!()
//     }

//     /// 下载文件块
//     async fn download_chunk<'chunk: 'static>(
//         &mut self,
//         start: u64,
//         end: u64,
//         part_num: usize,
//         cache_dir: &PathBuf,
//     ) -> Result<(), NovaMCoreError> {
//         let temp_path = cache_dir.join(format!("part_{}", part_num));
//             let mut file = File::options()
//                 .create(true)
//                 .append(true)
//                 .open(&temp_path)
//                 .await
//                 .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;
    
//             let mut retries = 0;
//             unsafe {
//                 let borrowed = pointer.lock().await;
//                 while retries < (*borrowed).max_retries {
//                     if (*borrowed).terminated.load(Ordering::SeqCst) {
//                         return Err(NovaMCoreError::msg("Download terminated"));
//                     }
        
//                     let response = match reqwest::ClientBuilder::new()
//                         .read_timeout(std::time::Duration::from_millis((*borrowed).timeout.load(Ordering::SeqCst)))
//                         //.default_headers(headers)
//                         .build()
//                         .expect("Failed to build network client.")
//                         .get(url.clone())
//                         .header(header::RANGE, format!("bytes={}-{}", start, end))
//                         .send()
//                         .await
//                     {
//                         Ok(r) => r,
//                         Err(_) => {
//                             retries += 1;
//                             continue;
//                         }
//                     };
        
//                     if !response.status().is_success() {
//                         retries += 1;
//                         continue;
//                     }
        
//                     let mut stream = response.bytes_stream();
//                     while let Some(chunk) = stream.next().await {
//                         let chunk = chunk.map_err(|e| NovaMCoreError::msg(&e.to_string()))?;
//                         file.write_all(&chunk)
//                             .await
//                             .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;
        
//                         // 更新进度
//                         (*borrowed).downloaded_bytes
//                             .fetch_add(chunk.len() as u64, Ordering::SeqCst);
//                         let progress = (*borrowed).progresser.lock().await;
//                         progress(
//                             Some((*borrowed).downloaded_bytes.load(Ordering::SeqCst)),
//                             Some((*borrowed).total_bytes.load(Ordering::SeqCst)),
//                             Some(chunk.len() as u64),
//                         );
//                     }
//                 }   
//                 return Ok(());
//             }
    
//             // Err(NovaMCoreError::msg(&format!(
//             //     "Failed after {} retries",
//             //     (*borrowed).max_retries
//             // )))
//     }

//     async fn merge_files(&mut self, cache_dir: &PathBuf) -> Result<(), NovaMCoreError> {
//         let mut dest_file = File::options()
//             .write(true)
//             .open(&self.dest)
//             .await
//             .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;

//         for i in 0..self.concurrency {
//             let part_path = cache_dir.join(format!("part_{}", i));
//             let mut part_file = File::open(&part_path)
//                 .await
//                 .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;

//             let mut content = Vec::new();
//             part_file
//                 .read_to_end(&mut content)
//                 .await
//                 .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;

//             dest_file
//                 .write_all(&content)
//                 .await
//                 .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;

//             tokio::fs::remove_file(part_path)
//                 .await
//                 .map_err(|e| NovaMCoreError::msg(&e.to_string()))?;
//         }

//         Ok(())
//     }

//     /// 终止下载
//     pub fn terminate(&self) {
//         self.terminated.store(true, Ordering::SeqCst);
//     }
// }
