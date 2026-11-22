use crate::mqtt::*;
use crate::plugins::*;
use crate::popup::open_popup_webview;
use crate::state::workspace::*;
use crate::state::{get_state, AppState};
use crate::state::{mark_workspace_dirty, save_workspace, save_workspace_as, open_workspace, get_dirty_workspaces, confirm_close};
use crate::state::{get_appearance, set_appearance, toggle_appearance, set_active_workspace, export_user_data, import_user_data};
use crate::state::persistence::StateStore;
use std::sync::Mutex;
use tauri::{Emitter, Manager, WindowEvent};

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
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let dirty = window.app_handle()
                        .state::<Mutex<AppState>>()
                        .lock()
                        .unwrap()
                        .dirty_workspaces
                        .clone();
                    if !dirty.is_empty() {
                        api.prevent_close();
                        let _ = window.emit(
                            "unsaved-work-warning",
                            serde_json::json!({"workspaces": dirty}),
                        );
                    }
                }
            }
        })
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let store = StateStore::new(app_data_dir)?;
            let (connections, workspaces, general, appearance) = store.load()?;

            let mut state = AppState::default();
            state.connections = connections.clone();
            state.workspaces = workspaces;
            state.general = general;
            state.appearance = appearance;
            app.manage(Mutex::new(state));
            app.manage(store);

            tauri::async_runtime::spawn(async move {
                restore_connections(connections).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_workspace,
            delete_workspace,
            rename_workspace,
            add_device_to_workspace,
            create_device_folder,
            rename_device_folder,
            delete_device_folder,
            move_device_to_folder,
            remove_device_from_workspace,
            update_device_parameters,
            add_connection_to_workspace,
            remove_connection_from_workspace,
            create_connection_folder,
            rename_connection_folder,
            delete_connection_folder,
            move_connection_to_folder,
            get_state,
            mark_workspace_dirty,
            save_workspace,
            save_workspace_as,
            open_workspace,
            get_dirty_workspaces,
            confirm_close,
            get_appearance,
            set_appearance,
            toggle_appearance,
            set_active_workspace,
            export_user_data,
            import_user_data,
            test_connection,
            create_connection,
            connection_activate,
            connection_deactivate,
            connection_remove,
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
    tauri_plugin_prevent_default::Builder::new()
        .platform(
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
        )
        .build()
}
