
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::core::utils::net;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifestOverall {
    pub latest: VersionManifestLatest,
    pub versions: Vec<VersionManifest>,
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
    BMCLAPI
}

impl DownloadSource {
    pub fn get_url(dsource:&Self ,url_path: &String) -> String{
        match dsource {
            Self::Mojang => "https://launchermeta.mojang.com/mc/game/".to_string() + url_path,
            Self::BMCLAPI => "https://bmclapi2.bangbang93.com/mc/game/".to_string() + url_path
        }
    }
}

pub async fn get_manifest_version_list() -> Result<VersionManifestOverall, Box<dyn Error>> {
    let client = net::HttpClient::new();
    let response =  client.get("https://launchermeta.mojang.com/mc/game/version_manifest.json").await.unwrap();
    let list: VersionManifestOverall = serde_json::from_str(&response.body).unwrap();
    Ok(list)
}