use std::sync::Mutex;
use crate::mqtt::*;
use crate::plugins::*;
use crate::popup::open_popup_webview;
use crate::state::{get_state, AppState};

mod mqtt;
mod plugins;
mod errors;
mod popup;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            get_state,
            gateway_connect,
            gateway_disconnect,
            gateway_publish,
            check_connection,
            update_devices,
            init_plugins,
            list_plugins,
            load_layout,
            get_panel_information,
            format_param_to_panel,
            format_panel_to_param,
            open_popup_webview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
