use super::NovaError;
use super::download::OnlineFetch;
use crate::utils::net;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use std::collections::HashMap;

use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::vec::Vec;

pub trait Contains {
    fn contains<T>(&self, str: &T) -> bool
    where
        T: ToString + ?Sized;
}

impl Contains for serde_json::Value {
    fn contains<T>(&self, str: &T) -> bool
    where
        T: ToString + ?Sized,
    {
        self.as_object().unwrap().contains_key(&str.to_string())
    }
}

pub trait MinecraftPredicate {
    fn of(&self) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifestOverall {
    pub latest: VersionManifestLatest,
    pub versions: Vec<VersionManifest>,
}

impl OnlineFetch for VersionManifestOverall {
    async fn fetch() -> Result<Self, NovaError> {
        let client = net::HttpClient::new();
        match client
            .get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
            .await
        {
            Ok(data) => {
                let list: VersionManifestOverall =
                    serde_json::from_str(&data.body.unwrap()).unwrap();
                Ok(list)
            }
            Err(e) => Err(NovaError::msg(&e.to_string())),
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

#[derive(Serialize, Deserialize, Debug)]
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

impl FromStr for VersionType {
    type Err = NovaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old_alpha" => Ok(VersionType::OldAlpha),
            "old_beta" => Ok(VersionType::OldBeta),
            "snapshot" => Ok(VersionType::Snapshot),
            "release" => Ok(VersionType::Release),
            _ => Err(NovaError::msg(&format!(
                "Unknown minecraft version type: {}",
                s
            ))),
        }
    }
}

impl ToString for VersionType {
    fn to_string(&self) -> String {
        match self {
            VersionType::OldAlpha => "old_alpha".to_string(),
            VersionType::OldBeta => "old_beta".to_string(),
            VersionType::Snapshot => "snapshot".to_string(),
            VersionType::Release => "release".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum NativeString {
    #[serde(rename = "natives-linux")]
    NativesLinux,
    #[serde(rename = "natives-osx")]
    NativesOSX,
    #[serde(rename = "natives-macos")]
    NativesMacOS,
    #[serde(rename = "natives-windows")]
    NativesWindows,
    #[serde(rename = "linux-x86_64")]
    NativesLinux64,
    #[serde(rename = "linux-aarch_64")]
    NativesLinuxAarch64,
    #[serde(rename = "natives-windows-x86")]
    NativesWindows32,
    #[serde(rename = "natives-macos-arm64")]
    NativesMacOSArm64,
    #[serde(rename = "natives-windows-arm64")]
    NativesWindowsArm64,
    #[serde(rename = "natives-macos-patch")]
    NativesMacOSPatch,
    #[serde(rename = "natives-windows-${arch}")]
    NativesWindowsArch,
}

#[derive(Clone)]
pub struct Asset {
    pub path: String,
    pub hash: String,
    pub size: u64,
}

#[derive(Clone)]
pub struct AssetObjects {
    pub vec: Vec<Asset>,
}

impl<'de> Deserialize<'de> for AssetObjects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_json = serde_json::Value::deserialize(deserializer)?;
        let json: HashMap<String, serde_json::Value> =
            serde_json::from_value(raw_json).expect("Cannot deserialize object.");
        let mut arr: Vec<Asset> = vec![];
        for (key, value) in json.iter() {
            arr.push(Asset {
                path: key.clone(),
                hash: value["hash"].to_string().replace('"', ""),
                size: value["size"].as_u64().unwrap_or(0),
            });
        }
        return Ok(Self { vec: arr });
    }
}

impl AssetObjects {
    pub fn iter(&self) -> AssetObjectIterator {
        return AssetObjectIterator::new(self);
    }
}

pub struct AssetObjectIterator {
    value: AssetObjects,
    ptr: usize,
}

impl AssetObjectIterator {
    fn new(value: &AssetObjects) -> Self {
        Self {
            value: value.clone(),
            ptr: 0,
        }
    }
}

impl Iterator for AssetObjectIterator {
    type Item = Asset;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr += 1;
        if self.ptr > self.value.vec.len().try_into().unwrap() {
            return None;
        }
        return Some(self.value.vec[self.ptr - 1].clone());
    }
}

impl MinecraftPredicate for Asset {
    fn of(&self) -> bool {
        return true;
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersion {
    pub arguments: Arguments,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: i32,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Logging,
    pub main_class: String,
    pub minimum_launcher_version: i32,
    pub release_time: String,
    pub time: String,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Simple(String),
    Conditional { rules: Vec<Rule>, value: ValueData },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueData {
    Simple(String),
    Conditional(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub action: String,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<HashMap<String, String>>,
}

impl MinecraftPredicate for Rule {
    fn of(&self) -> bool {
        let allowed = self.action == "allowed";
        if self.os.is_none() {
            return allowed;
        }
        let current_os: String = if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else if cfg!(target_os = "macos") {
            "osx".to_string()
        } else {
            "unknown".to_string()
        };
        let target_os = self.os.clone().unwrap();
        if target_os.contains_key("name") {
            if target_os.contains_key("version") {
                let target_os_version = target_os.get("version").unwrap().to_string();
                while target_os_version.contains(r#"\\"#) {
                    let _ = target_os_version.replace(r#"\\"#, "\\");
                }
                let current_os_version = sysinfo::System::os_version().unwrap();
                return target_os_version == current_os_version;
            }
            return allowed ^ (current_os == target_os.get("name").unwrap().to_string());
        }
        return false;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsRule {
    pub name: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub total_size: i64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    pub client: DownloadArtifact,
    pub client_mappings: Option<DownloadArtifact>,
    pub server: DownloadArtifact,
    pub server_mappings: Option<DownloadArtifact>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadArtifact {
    pub sha1: String,
    pub size: i64,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: i32,
}

#[derive(Debug)]
pub struct Library {
    pub name: String,
    pub native: bool,
    pub native_strings: Vec<NativeString>,
    pub rules: Option<Vec<Rule>>,
    pub downloads: HashMap<String, DownloadArtifact>,
}

impl<'de> Deserialize<'de> for Library {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_json = serde_json::Value::deserialize(deserializer)?;
        let json: HashMap<String, serde_json::Value> =
            serde_json::from_value(raw_json).expect("Failed to deserialize MC Library Index.");
        let mut native = false;
        let mut native_strings: Vec<NativeString> = vec![];
        let mut name: String = String::new();
        let mut rules: Option<Vec<Rule>> = None;
        if json.contains_key("rules") {
            rules = Some(
                serde_json::from_value(json["rules"].clone()).expect("Failed to fetch rules."),
            );
        }
        for (key, value) in json.iter() {
            if key == "natives" {
                native = true;
                let raw_native_strings: HashMap<String, serde_json::Value> =
                    serde_json::from_value(value.clone()).expect("Failed to fetch native strings.");
                for (_key, native_string) in raw_native_strings {
                    native_strings.push(
                        serde_json::from_value(native_string)
                            .expect("Failed to fetch native strings."),
                    );
                }
            } else if key == "name" {
                name = value.to_string().replace('"', "");
                let splited_name: Vec<_> = name.split(':').collect();
                if splited_name.len() >= 4 {
                    native_strings.push(
                        serde_json::from_str(splited_name[3])
                            .expect("Failed to fetch native strings."),
                    );
                }
            }
        }
        if native {
            if json["downloads"].contains("classifiers") {
                let downloads: HashMap<String, DownloadArtifact> =
                    serde_json::from_value(json["downloads"]["classifiers"].clone())
                        .expect("Failed to fetch classifiers.");
                return Ok(Self {
                    name,
                    native,
                    native_strings,
                    rules,
                    downloads,
                });
            } else if json["downloads"].contains("artifact") {
                let artifacts: DownloadArtifact =
                    serde_json::from_value(json["downloads"]["artifact"].clone())
                        .expect("Failed to fetch artifacts.");
                let mut downloads: HashMap<String, DownloadArtifact> = HashMap::new();
                let native_string = serde_json::to_value(native_strings[0])
                    .expect("Failed to fetch native strings.")
                    .to_string()
                    .replace('"', "");
                downloads.insert(native_string, artifacts);
                return Ok(Self {
                    name,
                    native,
                    native_strings,
                    rules,
                    downloads,
                });
            }
        } else if json["downloads"].contains("artifact") {
            let artifacts: DownloadArtifact =
                serde_json::from_value(json["downloads"]["artifact"].clone())
                    .expect("Failed to fetch artifacts.");
            let mut downloads: HashMap<String, DownloadArtifact> = HashMap::new();
            downloads.insert("artifact".to_string(), artifacts);
            return Ok(Self {
                name,
                native,
                native_strings,
                rules,
                downloads,
            });
        }
        Err(Error::custom("Failed to deserialize Library Index."))
    }
}

impl MinecraftPredicate for Library {
    fn of(&self) -> bool {
        if !self.native {
            return true;
        }
        let mut rule_allowed = true;
        if let Some(rules) = &self.rules {
            for rule in rules {
                rule_allowed &= rule.of();
            }
        }
        if !rule_allowed {
            return false;
        }
        let lwjgl3_regex = regex::Regex::new("org.lwjgl:lwjgl(-[a-z._.\\-.0-9]*)?:3.[0-9]*.[0-9]*(-[a-z.0-9._.\\-]*)?:([a-z._.\\-.0-9]*)?").unwrap();
        let lwjgl3 = lwjgl3_regex.is_match(&self.name);
        let mut matched_natives: Vec<NativeString> = Vec::new();
        if cfg!(target_os = "windows") {
            if cfg!(target_arch = "x86") {
                if lwjgl3 {
                    matched_natives = vec![
                        NativeString::NativesWindows,
                        NativeString::NativesWindows32,
                        NativeString::NativesWindowsArch,
                    ]
                } else {
                    matched_natives = vec![
                        NativeString::NativesWindows32,
                        NativeString::NativesWindowsArch,
                    ]
                }
            } else if cfg!(target_arch = "x86_64") {
                matched_natives = vec![
                    NativeString::NativesWindows,
                    NativeString::NativesWindowsArch,
                ]
            } else if cfg!(target_arch = "aarch64") {
                matched_natives = vec![NativeString::NativesWindowsArm64]
            }
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "x86_64") {
                matched_natives = vec![NativeString::NativesMacOS, NativeString::NativesMacOSPatch]
            } else if cfg!(target_arch = "aarch64") {
                matched_natives = vec![NativeString::NativesMacOSArm64]
            }
        } else if cfg!(target_os = "linux") {
            if cfg!(target_arch = "x86_64") {
                matched_natives = vec![NativeString::NativesLinux, NativeString::NativesLinux64]
            } else if cfg!(target_arch = "aarch64") {
                matched_natives = vec![NativeString::NativesLinuxAarch64]
            }
        }
        if matched_natives.len() == 0 {
            return false;
        }

        for target in self.native_strings.iter() {
            for current in matched_natives.iter() {
                if *target == *current {
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<DownloadArtifact>,
    pub classifiers: Option<HashMap<String, DownloadArtifact>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingFile,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Clone)]
pub struct GamePath {
    pub path: PathBuf,
}

impl FromStr for GamePath {
    type Err = NovaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        Ok(Self { path })
    }
}

impl Display for GamePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap_or(""))
    }
}

impl GamePath {
    pub fn init(&self) -> Result<(), NovaError> {
        Ok(())
    }
}
