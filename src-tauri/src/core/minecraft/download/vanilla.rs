use std::string;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct VersionManifestOverall {
    latest: VersionManifestLatest,
    versions: Vec<VersionManifest>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionManifestLatest {
    release: String,
    snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionManifest {
    id: String,
    #[serde(rename = "type")]
    version_type: VersionType,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
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

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "old_alpha" => Ok(VersionType::OldAlpha),
            "old_beta" => Ok(VersionType::OldBeta),
            "snapshot" => Ok(VersionType::Snapshot),
            "release" => Ok(VersionType::Release),
            _ => Err(format!("Unknown version type: {}", s)),
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