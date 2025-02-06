use pcl2_nova_app_lib::core::utils::downloader::{
    DownloadConfig, DownloadManager, DownloadManagerConfig, Downloader,
};
use std::time::Duration;
use tokio;

#[test]
fn test_downloader() {
    let dl = Downloader::new(DownloadConfig{
        url: String::from("https://piston-meta.mojang.com/v1/packages/0405359a694f22b6423f4d64bb828ea732db4d33/19.json"),
        output_path: dirs_next::desktop_dir().unwrap().join("test.json"),
        temp_dir: dirs_next::cache_dir().unwrap().join("PCL-Nova").join("cache"),
        max_retries: 5,
        timeout: Duration::from_millis(30_000),
        max_threads: 1
    }).unwrap();
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(dl.start());
}

#[test]
fn test_muiltdownload() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let dls = vec![
            DownloadManagerConfig{
                url: String::from("https://libraries.minecraft.net/com/mojang/datafixerupper/8.0.16/datafixerupper-8.0.16.jar"),
                dest: dirs_next::desktop_dir().unwrap().join("1.jar"),
                max_retries: 3,
                max_threads: 1,
                timeout_secs: 30,
            },
            DownloadManagerConfig{
                url: String::from("https://libraries.minecraft.net/com/mojang/jtracy/1.0.29/jtracy-1.0.29.jar"),
                dest: dirs_next::desktop_dir().unwrap().join("2.jar"),
                max_retries: 3,
                max_threads: 1,
                timeout_secs: 30,
            }
        ];
        let dl = DownloadManager::new(&dls).unwrap();
        dl.download_all().await;
    })
}
