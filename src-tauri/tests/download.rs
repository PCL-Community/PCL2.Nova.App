use std::{path::PathBuf, time::Duration};

use pcl2_nova_app_lib::core::utils::downloader::{DownloadConfig,Downloader};


#[test]
fn test_downloader(){
    let dl = Downloader::new(DownloadConfig{
        url: String::from("https://piston-meta.mojang.com/v1/packages/0405359a694f22b6423f4d64bb828ea732db4d33/19.json"),
        output_path: PathBuf::from("D:\\test\\a.json"),
        temp_dir: PathBuf::from("D:\\test\\temp"),
        max_retries: 5,
        timeout: Duration::from_millis(30_000),
        max_threads: 8
    }).unwrap();
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(dl.start());
}