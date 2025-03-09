use std::{io::{Read, Write}, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::NovaError;


#[derive(Deserialize, Serialize)]
pub enum VersionIsolation {
    Off,
    All,
    Mod,
    Unofficial,
    ModAndUnofficial,
}

#[derive(Deserialize, Serialize)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version_isolation: VersionIsolation,
    pub custom_title: String, 
    pub theme_mode: ThemeMode,
}

impl Config {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version_isolation: VersionIsolation::Off,
            custom_title: String::new(),
            theme_mode: ThemeMode::System
        }
    }

    pub fn from_global() -> Result<Config, NovaError> {
        let config_path = 
            dirs_next::config_dir().unwrap().join("PCLNova").join("config.json");
        if !config_path.is_file() {
            Self::init_global()?;
        }
        let mut file = std::fs::File::open(config_path).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        serde_json::from_str(&buf).map_err(|e| {
            NovaError::msg(&e.to_string())
        })
    }

    pub fn to_global(config: &Self) -> Result<(), NovaError> {
        let config_path = 
            dirs_next::config_dir().unwrap().join("PCLNova").join("config.json");
        if !config_path.is_file() {
            Self::init_global()?;
        }
        let mut file = std::fs::File::open(config_path).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        let buf = serde_json::to_string(config).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        file.write_all(buf.as_bytes()).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        Ok(())
    }

    pub fn init_global() -> Result<(), NovaError> {
        let config_path = 
            dirs_next::config_dir().unwrap().join("PCLNova");
        std::fs::create_dir_all(&config_path).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        let mut file = std::fs::File::create(config_path.join("config.json")).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        file.write_all(serde_json::to_string(&Self::new()).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?.as_bytes()).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        Ok(())
    }

    pub fn from_local(path: &PathBuf) -> Result<Config, NovaError> {
        let config_path = 
            path.join("nova_config.json");
        if !config_path.is_file() {
            Self::init_local(path)?;
        }
        let mut file = std::fs::File::open(config_path).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        serde_json::from_str(&buf).map_err(|e| {
            NovaError::msg(&e.to_string())
        })
    }

    pub fn to_local(config: &Self, path: &PathBuf) -> Result<(), NovaError> {
        let config_path = 
            path.join("nova_config.json");
        if !config_path.is_file() {
            Self::init_local(path)?;
        }
        let mut file = std::fs::File::open(config_path).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        let buf = serde_json::to_string(config).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        file.write_all(buf.as_bytes()).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        Ok(())
    }

    pub fn init_local(path: &PathBuf) -> Result<(), NovaError> {
        std::fs::create_dir_all(path).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        let mut file = std::fs::File::create(path.join("nova_config.json")).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        file.write_all(serde_json::to_string(&Self::new()).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?.as_bytes()).map_err(|e| {
            NovaError::msg(&e.to_string())
        })?;
        Ok(())
    }
}