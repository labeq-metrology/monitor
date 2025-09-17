use crate::mqtt::*;
use crate::plugins::*;
use tauri::{AppHandle, Manager};

mod mqtt;
mod plugins;
mod errors;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
struct MonitorData {
    gateway_connected: bool,
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
            init_plugins
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
