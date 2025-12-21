use crate::mqtt::*;
use crate::plugins::*;
use crate::settings::open_settings;

mod mqtt;
mod plugins;
mod errors;
mod settings;

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
            gateway_publish,
            check_connection,
            init_plugins,
            list_plugins,
            load_layout,
            open_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
