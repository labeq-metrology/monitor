use crate::mqtt::*;
use crate::plugins::*;
use crate::popup::open_popup_webview;
use crate::state::{get_state, AppState};
use std::sync::Mutex;
mod errors;
mod mqtt;
mod plugins;
mod popup;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default())
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

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD))
        .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::Builder::new().platform(
        PlatformOptions::new()
            // Whether general form information should be saved and autofilled.
            .general_autofill(false)
            // Whether password information should be autosaved.
            .password_autosave(false)
            // Whether browser-specific accelerator keys are enabled.
            .browser_accelerator_keys(false)
            // Whether the default context menus are shown in the webview.
            .default_context_menus(false)
            // Whether the webview renders the default JavaScript dialog box.
            .default_script_dialogs(false),
    ).build()
}
