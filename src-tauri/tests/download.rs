use std::{path::PathBuf, time::Duration};
use tokio;
use pcl2_nova_app_lib::core::utils::downloader::{DownloadConfig, DownloadManager, DownloadManagerConfig, Downloader};


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

#[test]
fn test_muiltdownload(){
    let dls = vec![
        DownloadManagerConfig{
            url: String::from("https://libraries.minecraft.net/com/mojang/datafixerupper/8.0.16/datafixerupper-8.0.16.jar"),
            dest: dirs_next::desktop_dir().unwrap()
        },
        DownloadManagerConfig{
            url: String::from("https://libraries.minecraft.net/com/mojang/jtracy/1.0.29/jtracy-1.0.29.jar"),
            dest: dirs_next::desktop_dir().unwrap()
        }
    ];
    let dl = DownloadManager::new(&dls).unwrap();
    dl.start();
    dl.wait_for_end();
}