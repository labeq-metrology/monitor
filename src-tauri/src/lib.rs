use crate::mqtt::*;
use std::fs;
use tauri::{AppHandle, Manager};
use tauri::utils::platform::resource_dir;

mod mqtt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
struct MonitorData {
    gateway_connected: bool,
}

#[tauri::command]
fn load_plugin(app: AppHandle, path: String) -> Result<String, String> {
    let plugin_path = app.path().resource_dir()
        .expect("Failed to get resource dir")
        .join(path);
    fs::read_to_string(plugin_path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            gateway_connect,
            gateway_disconnect,
            check_connection,
            load_plugin
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
