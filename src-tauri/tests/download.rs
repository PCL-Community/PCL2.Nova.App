use std::{path::PathBuf, time::Duration};

use pcl2_nova_app_lib::core::utils::downloader::{DownloadConfig,Downloader};


#[test]
fn test_downloader(){
    let dl = Downloader::new(DownloadConfig{
        url: String::from("https://piston-meta.mojang.com/v1/packages/0405359a694f22b6423f4d64bb828ea732db4d33/19.json"),
        output_path: dirs_next::desktop_dir().unwrap().join("test.json"),
        temp_dir: dirs_next::cache_dir().unwrap().join("PCL-Nova").join("cache"),
        max_retries: 5,
        timeout: Duration::from_millis(30_000),
        max_threads: 8
    }).unwrap();
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(dl.start());
}