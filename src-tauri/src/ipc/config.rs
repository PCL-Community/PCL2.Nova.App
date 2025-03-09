use crate::config::*;
use std::path::PathBuf;

#[tauri::command]
pub async fn get_config_global() -> Result<Config, String> {
    Config::from_global().map_err(|e| { e.to_string() })
}

#[tauri::command]
pub async fn write_config_global(config: Config) -> Result<(), String> {
    Config::to_global(&config).map_err(|e| { e.to_string() })
}

#[tauri::command]
pub async fn get_config_local(path: String) -> Result<Config, String> {
    Config::from_local(&PathBuf::from(path)).map_err(|e| { e.to_string() })
}

#[tauri::command]
pub async fn write_config_local(config: Config, path: String) -> Result<(), String> {
    Config::to_local(&config, &PathBuf::from(path)).map_err(|e| { e.to_string() })
}