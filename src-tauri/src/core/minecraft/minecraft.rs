use std::collections::HashMap;
use serde::{{de::{self, Error}}, Deserialize, Deserializer, Serialize};

use std::vec::Vec;



pub trait Contains {
    fn contains<T>(&self, str: &T) -> bool where T: ToString + ?Sized;
}

impl Contains for serde_json::Value {
    fn contains<T>(&self, str: &T) -> bool where T: ToString + ?Sized {
        self.as_object().unwrap().contains_key(&str.to_string())
    }
}

pub trait MinecraftPredicate {
    fn of(&self) -> bool;
}

#[derive(Clone)]
pub struct MinecraftAsset {
    pub path: String,
    pub hash: String,
    pub size: u64
}

#[derive(Clone)]
pub struct MinecraftAssetObjects {
    pub vec: Vec<MinecraftAsset>
}

impl<'de> Deserialize<'de> for MinecraftAssetObjects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let raw_json = serde_json::Value::deserialize(deserializer)?;
        let json: HashMap<String, serde_json::Value> = serde_json::from_value(raw_json).expect("Cannot deserialize object.");
        let mut arr: Vec<MinecraftAsset> = vec![];
        for (key, value) in json.iter() {
            arr.push(MinecraftAsset { path: key.clone(), hash: value["hash"].to_string().replace('"', ""), size: value["size"].as_u64().unwrap_or(0) });
        }
        return Ok(Self { vec: arr });
    }
}

impl MinecraftAssetObjects {
    pub fn iter(&self) -> AssetObjectIterator { 
        return AssetObjectIterator::new(self);
    }
}

pub struct AssetObjectIterator {
    value: MinecraftAssetObjects,
    ptr: usize
}

impl AssetObjectIterator {
    fn new(value: &MinecraftAssetObjects) -> Self {
        Self { value: value.clone(), ptr: 0 }
    }
}

impl Iterator for AssetObjectIterator {
    type Item = MinecraftAsset;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr += 1;
        if self.ptr > self.value.vec.len().try_into().unwrap() {
            return None;
        }
        return Some(self.value.vec[self.ptr - 1].clone());
    }
}


impl MinecraftPredicate for MinecraftAsset {
    fn of(&self) -> bool {
        return true;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftOSRule {
    pub action: String,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<HashMap<String, String>>
}

impl MinecraftPredicate for MinecraftOSRule {
    fn of(&self) -> bool {
        let allowed = self.action == "allowed";
        if self.os.is_none() {
            return allowed;
        }
        let current_os: String = 
            if cfg!(target_os = "windows") {
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

pub struct MinecraftLibrary {
    name: String,
    path: String,
    url: String,
    native: bool,
    native_string: Option<String>
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
        let current_os: String = 
            if cfg!(target_os = "windows") {
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
    pub native_strings: Vec<String>,
    pub rules: Option<Vec<Rule>>,
    pub downloads: HashMap<String, DownloadArtifact>
}

impl<'de> Deserialize<'de> for Library {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> 
    {
        let raw_json = serde_json::Value::deserialize(deserializer)?;
        let json: HashMap<String, serde_json::Value> = serde_json::from_value(raw_json).expect("Failed to deserialize MC Library Index.");
        let mut native = false;
        let mut native_strings: Vec<String> = vec![];
        let mut name: String = String::new();
        let mut rules: Option<Vec<Rule>> = None;
        if json.contains_key("rules") {
            rules = Some(serde_json::from_value(json["rules"].clone()).expect("Failed to fetch rules."));
        }
        for (key, value) in json.iter() {
            if key == "natives" {
                native = true;
                let raw_native_strings = value.as_array();
                if let Some(rns) = raw_native_strings {
                    for native_string in rns {
                        native_strings.push(native_string.to_string().replace('"', ""));
                    }
                }
            } else if key == "name" {
                name = value.to_string().replace('"', "");
                let splited_name: Vec<_> = name.split(':').collect();
                if splited_name.len() >= 4 {
                    native_strings.push(splited_name[3].to_string());
                }
            }
        }
        if native {
            if json["downloads"].contains("classifiers") {
                let downloads: HashMap<String, DownloadArtifact> = serde_json::from_value(json["downloads"]["classifiers"].clone()).expect("Failed to fetch classifiers.");
                return Ok(Self { name, native, native_strings, rules, downloads });
            } else if json["downloads"].contains("artifact") {
                let artifacts: DownloadArtifact = serde_json::from_value(json["downloads"]["artifact"].clone()).expect("Failed to fetch artifacts.");
                let mut downloads: HashMap<String, DownloadArtifact> = HashMap::new();
                downloads.insert(native_strings[0].clone(), artifacts);
                return Ok(Self { name, native, native_strings, rules, downloads });
            }
        } else if json["downloads"].contains("artifact") {
            let artifacts: DownloadArtifact = serde_json::from_value(json["downloads"]["artifact"].clone()).expect("Failed to fetch artifacts.");
            let mut downloads: HashMap<String, DownloadArtifact> = HashMap::new();
            downloads.insert("artifact".to_string(), artifacts);
            return Ok(Self { name, native, native_strings, rules, downloads });
        }
        Err(Error::custom("Failed to deserialize Library Index."))
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
