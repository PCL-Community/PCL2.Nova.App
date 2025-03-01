pub mod core;

use core::minecraft::auth::{device_auth, user_login};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![device_auth, user_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}