use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::iter::Map;

pub trait Contains {
    fn contains<T>(&self, str: &T) -> bool where T: ToString;
}

impl Contains for serde_json::Value {
    fn contains<T>(&self, str: &T) -> bool where T: ToString {
        self.as_object().unwrap().contains_key(&str.to_string())
    }
}

pub trait MinecraftPredicate {
    fn of(&self) -> bool;
}

#[derive(Clone)]
pub struct MinecraftAsset {
    path: String,
    hash: u64,
    size: u64
}

impl MinecraftAsset {
    pub fn from_asset_index(json: serde_json::Value) -> Vec<Self> {
        let hashmap: HashMap<String, serde_json::Value> = serde_json::from_value(json).expect("Cannot convert JSON into HashMap.");
        let mut arr: Vec<Self> = Vec::new();
        for (key, value) in hashmap {
            arr.push(
                MinecraftAsset {
                    path: key.to_string(),
                    hash: value["hash"].as_u64().expect("Cannot convert to u64."),
                    size: value["size"].as_u64().expect("Cannot convert to u64.")
                }
            );
        }
        return arr;
    }
}

impl MinecraftPredicate for MinecraftAsset {
    fn of(&self) -> bool {
        return true;
    }
}

#[derive(Serialize, Deserialize)]
pub struct MinecraftOSRule {
    action: String,
    os: Option<HashMap<String, String>>
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