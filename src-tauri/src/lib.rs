pub mod config;
pub mod core;
pub mod ipc;
pub mod auth;

use crate::ipc::login::{device_auth, user_login};
use crate::ipc::config::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            device_auth, 
            user_login,
            get_config_global,
            write_config_global,
            get_config_local,
            write_config_local
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
