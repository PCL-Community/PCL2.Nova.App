use crate::core::minecraft::file_struct::version_asset_struct;
use crate::core::utils::downloader::DownloadManagerConfig;
use crate::core::utils::{downloader, net};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::time::Duration;
use std::{fs, thread};
use std::ops::Not;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifestOverall {
    pub latest: VersionManifestLatest,
    pub versions: Vec<VersionManifest>,
}

impl VersionManifestOverall {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client = net::HttpClient::new();
        match client
            .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .await
        {
            Ok(data) => {
                let list: VersionManifestOverall = serde_json::from_str(&data.body).unwrap();
                Ok(list)
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

impl VersionManifest {
    pub async fn download(&self, dest: PathBuf) -> Result<(), Box<dyn Error>> {
        // 下载实例 Json 文件
        let _temp = PathBuf::from(&self.url);
        let filename = match _temp.file_name() {
            Some(name) => name,
            None => return Err(Box::from("No filename given")),
        };
        let config = vec![downloader::DownloadManagerConfig {
            url: self.url.clone(),
            dest: dest.join(filename),
            max_threads: 2,
            max_retries: 5,
            timeout_secs: 30,
        }];
        let dl = downloader::DownloadManager::new(&config).unwrap();
        dl.download_all().await;

        drop(dl);
        drop(config);

        let filecontent = match fs::read_to_string(dest.join(filename).to_str().unwrap()) {
            Ok(x) => x.to_string(),
            Err(e) => return Err(Box::from(format!("Read file content failed: {}", e))),
        };

        // 读取 Json 文件中所需的 lib 下载
        let v_asset: version_asset_struct::MinecraftVersion =
            match serde_json::from_str(filecontent.as_str()) {
                Ok(x) => x,
                Err(e) => return Err(Box::from(format!("Deserialize json file failed: {}", e))),
            };

        let mut config: Vec<DownloadManagerConfig> = Vec::new();
        config.push(DownloadManagerConfig {
            url: v_asset.downloads.client.url,
            dest: dest.join("client.jar"),
            max_threads: 8,
            max_retries: 3,
            timeout_secs: 30,
        });
        for (_, library) in v_asset.libraries.iter().enumerate() {
            if let Some(artifact) = &library.downloads.artifact {
                if let Some(save_path) = artifact.path.clone() {
                    let new_config = DownloadManagerConfig {
                        url: artifact.url.clone(),
                        dest: dest.join("libraries").join(save_path),
                        max_threads: 1,
                        max_retries: 3,
                        timeout_secs: 30,
                    };
                    config.push(new_config);
                }
            }
        }
        let dl = downloader::DownloadManager::new(&config).unwrap();
        dl.download_all().await;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionType {
    #[serde(rename = "old_alpha")]
    OldAlpha,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "release")]
    Release,
}

impl VersionType {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        match s {
            "old_alpha" => Ok(VersionType::OldAlpha),
            "old_beta" => Ok(VersionType::OldBeta),
            "snapshot" => Ok(VersionType::Snapshot),
            "release" => Ok(VersionType::Release),
            _ => Err(format!("Unknown minecraft version type: {}", s).into()),
        }
    }

    fn to_string(&self) -> String {
        match self {
            VersionType::OldAlpha => "old_alpha".to_string(),
            VersionType::OldBeta => "old_beta".to_string(),
            VersionType::Snapshot => "snapshot".to_string(),
            VersionType::Release => "release".to_string(),
        }
    }
}

pub enum DownloadSource {
    Mojang,
    BMCLAPI,
}

impl DownloadSource {
    pub fn get_url(dsource: &Self, url_path: &String) -> String {
        match dsource {
            Self::Mojang => "https://launchermeta.mojang.com/mc/game/".to_string() + url_path,
            Self::BMCLAPI => "https://bmclapi2.bangbang93.com/mc/game/".to_string() + url_path,
        }
    }
}
