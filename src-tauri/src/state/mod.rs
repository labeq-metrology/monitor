pub mod connection;
pub mod workspace;
pub mod persistence;

use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use crate::state::connection::ConnectionData;
use crate::state::workspace::LabeqWorkspace;
use crate::state::workspace::Item;
use crate::state::persistence::{AppearanceState, GeneralState, StateStore};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppState {
    pub connections: Vec<ConnectionData>,
    pub workspaces: Vec<LabeqWorkspace>,
    pub general: GeneralState,
    pub appearance: AppearanceState,
    #[serde(default)]
    pub dirty_workspaces: Vec<String>,
    #[serde(default)]
    pub workspace_files: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MissingWorkspaceDevice {
    pub connection_id: String,
    pub device_uuid: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceLoadReport {
    pub workspace_id: String,
    pub workspace_name: String,
    pub missing_connections: Vec<String>,
    pub missing_devices: Vec<MissingWorkspaceDevice>,
}

fn collect_connection_ids(items: &[Item<String>], output: &mut Vec<String>) {
    for item in items {
        match item {
            Item::Item(id) => output.push(id.clone()),
            Item::Folder(folder) => collect_connection_ids(&folder.items, output),
        }
    }
}

fn collect_device_ids(
    items: &[Item<(String, String)>],
    output: &mut Vec<(String, String)>,
) {
    for item in items {
        match item {
            Item::Item((connection_id, device_uuid)) => {
                output.push((connection_id.clone(), device_uuid.clone()));
            }
            Item::Folder(folder) => collect_device_ids(&folder.items, output),
        }
    }
}

pub fn update_state(app: &AppHandle, f: impl FnOnce(&mut AppState)) {
    let state = app.state::<Mutex<AppState>>();
    let state_clone = {
        let mut lock = state.lock().unwrap();
        f(&mut lock);
        lock.clone()
    };

    if let Some(store) = app.try_state::<StateStore>() {
        if let Err(error) = store.save_recovery_workspaces(&state_clone.workspaces) {
            let _ = app.emit(
                "state-save-error",
                serde_json::json!({"operation": "workspaces", "error": error.to_string()}),
            );
        }
        for connection in &state_clone.connections {
            if let Err(error) = store.save_connection(connection) {
                let _ = app.emit(
                    "state-save-error",
                    serde_json::json!({"operation": "connections", "error": error.to_string()}),
                );
            }
        }
    }

    app.emit("app-state-update", state_clone)
        .expect("Failed to emit state update event");
}

#[tauri::command]
pub fn get_state(app: AppHandle) -> AppState {
    app.state::<Mutex<AppState>>().lock().unwrap().clone()
}

pub fn update_workspace_state(app: &AppHandle, workspace_id: &str, f: impl FnOnce(&mut AppState)) {
    let state = app.state::<Mutex<AppState>>();
    let state_clone = {
        let mut lock = state.lock().unwrap();
        f(&mut lock);
        if !lock.dirty_workspaces.contains(&workspace_id.to_string()) {
            lock.dirty_workspaces.push(workspace_id.to_string());
        }
        lock.clone()
    };

    if let Some(store) = app.try_state::<StateStore>() {
        if let Err(error) = store.save_recovery_workspaces(&state_clone.workspaces) {
            let _ = app.emit(
                "state-save-error",
                serde_json::json!({"operation": "workspace-recovery", "error": error.to_string()}),
            );
        }
    }

    app.emit("app-state-update", state_clone)
        .expect("Failed to emit state update event");
}

#[tauri::command]
pub fn get_appearance(app: AppHandle) -> AppearanceState {
    app.state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .appearance
        .clone()
}

#[tauri::command]
pub fn set_appearance(
    app: AppHandle,
    color_palette: String,
    mode: String,
) -> Result<(), String> {
    let store = app
        .try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;
    let appearance = AppearanceState {
        schema_version: 1,
        color_palette,
        mode,
    };
    store
        .save_appearance(&appearance)
        .map_err(|error| error.to_string())?;
    {
        let state_handle = app.state::<Mutex<AppState>>();
        let mut state = state_handle.lock().unwrap();
        state.appearance = appearance;
    }
    let state_handle = app.state::<Mutex<AppState>>();
    let appearance = state_handle
        .lock()
        .unwrap()
        .appearance
        .clone();
    let payload = serde_json::json!({
        "palette": appearance.color_palette,
        "mode": appearance.mode,
    });
    let _ = app.emit("active-theme-change", payload);
    Ok(())
}

#[tauri::command]
pub fn toggle_appearance(app: AppHandle) -> Result<AppearanceState, String> {
    let store = app
        .try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;

    let appearance = {
        let state_handle = app.state::<Mutex<AppState>>();
        let mut state = state_handle.lock().unwrap();
        let mode = if state.appearance.mode == "dark" {
            "light"
        } else {
            "dark"
        };
        let appearance = AppearanceState {
            schema_version: 1,
            color_palette: state.appearance.color_palette.clone(),
            mode: mode.to_string(),
        };
        state.appearance = appearance.clone();
        appearance
    };

    store
        .save_appearance(&appearance)
        .map_err(|error| error.to_string())?;

    let payload = serde_json::json!({
        "palette": appearance.color_palette,
        "mode": appearance.mode,
    });
    let _ = app.emit("active-theme-change", payload);
    Ok(appearance)
}

#[tauri::command]
pub fn set_active_workspace(
    app: AppHandle,
    workspace_id: Option<String>,
) -> Result<(), String> {
    let store = app
        .try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;
    let general = GeneralState {
        schema_version: 1,
        active_workspace_id: workspace_id,
    };
    store
        .save_general_state(&general)
        .map_err(|error| error.to_string())?;
    app.state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .general = general;
    Ok(())
}

#[tauri::command]
pub fn mark_workspace_dirty(app: AppHandle, workspace_id: String) -> Result<(), String> {
    let state_handle = app.state::<Mutex<AppState>>();
    let mut state = state_handle.lock().unwrap();
    if !state.dirty_workspaces.contains(&workspace_id) {
        state.dirty_workspaces.push(workspace_id);
    }
    let state_clone = state.clone();
    drop(state);
    app.emit("app-state-update", state_clone)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_workspace(app: AppHandle, workspace_id: String) -> Result<String, String> {
    let state_handle = app.state::<Mutex<AppState>>();
    let (workspace, existing_path) = {
        let state = state_handle.lock().unwrap();
        let workspace = state.workspaces
            .iter()
            .find(|workspace| workspace.id == workspace_id)
            .cloned()
            .ok_or_else(|| "Workspace not found".to_string())?;
        let path = state.workspace_files.get(&workspace_id).cloned();
        (workspace, path)
    };
    let store = app.try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;

    if let Some(path) = existing_path {
        store.save_workspace_file(&workspace, std::path::Path::new(&path))
            .map_err(|error| error.to_string())?;
        store.save_workspace(&workspace)
            .map_err(|error| error.to_string())?;
        store.clear_workspace_recovery(&workspace_id)
            .map_err(|error| error.to_string())?;
        store.clear_workspace_recovery(&workspace_id)
            .map_err(|error| error.to_string())?;
        let state_handle = app.state::<Mutex<AppState>>();
        let mut state = state_handle.lock().unwrap();
        state.dirty_workspaces.retain(|id| id != &workspace_id);
        app.emit("app-state-update", state.clone())
            .map_err(|error| error.to_string())?;
        return Ok(path);
    }
    save_workspace_as(app, workspace_id)
}

#[tauri::command]
pub fn save_workspace_as(app: AppHandle, workspace_id: String) -> Result<String, String> {
    let state_handle = app.state::<Mutex<AppState>>();
    let workspace = {
        let state = state_handle.lock().unwrap();
        state.workspaces
            .iter()
            .find(|workspace| workspace.id == workspace_id)
            .cloned()
            .ok_or_else(|| "Workspace not found".to_string())?
    };
    let path = rfd::FileDialog::new()
        .set_file_name(&format!("{}.labeq-workspace.json", workspace.name))
        .add_filter("LabEq workspace", &["json"])
        .save_file()
        .ok_or_else(|| "Save cancelled".to_string())?;
    let path_string = path.to_string_lossy().to_string();
    let store = app.try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;
    store.save_workspace_file(&workspace, &path)
        .map_err(|error| error.to_string())?;
    store.save_workspace(&workspace)
        .map_err(|error| error.to_string())?;
    store.clear_workspace_recovery(&workspace_id)
        .map_err(|error| error.to_string())?;

    let state_handle = app.state::<Mutex<AppState>>();
    let mut state = state_handle.lock().unwrap();
    state.workspace_files.insert(workspace_id.clone(), path_string.clone());
    state.dirty_workspaces.retain(|id| id != &workspace_id);
    app.emit("app-state-update", state.clone())
        .map_err(|error| error.to_string())?;
    Ok(path_string)
}

#[tauri::command]
pub fn open_workspace(app: AppHandle) -> Result<WorkspaceLoadReport, String> {
    let path = rfd::FileDialog::new()
        .add_filter("LabEq workspace", &["json"])
        .pick_file()
        .ok_or_else(|| "Open cancelled".to_string())?;
    let path_string = path.to_string_lossy().to_string();
    let store = app.try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;
    let workspace = store.load_workspace_file(&path)
        .map_err(|error| error.to_string())?;
    store.save_workspace(&workspace)
        .map_err(|error| error.to_string())?;
    let workspace_id = workspace.id.clone();

    let state_handle = app.state::<Mutex<AppState>>();
    let report = {
        let mut state = state_handle.lock().unwrap();
        let missing_connections = {
            let mut ids = Vec::new();
            collect_connection_ids(&workspace.connections.items, &mut ids);
            ids.sort();
            ids.dedup();
            ids.into_iter()
                .filter(|id| !state.connections.iter().any(|connection| &connection.id == id))
                .collect::<Vec<_>>()
        };
        let missing_devices = {
            let mut devices = Vec::new();
            collect_device_ids(&workspace.devices.items, &mut devices);
            devices
                .into_iter()
                .filter_map(|(connection_id, device_uuid)| {
                    let exists = state.connections.iter().any(|connection| {
                        connection.id == connection_id
                            && connection.devices.iter().any(|device| device.uuid == device_uuid)
                    });
                    if exists {
                        None
                    } else {
                        Some(MissingWorkspaceDevice {
                            connection_id,
                            device_uuid,
                        })
                    }
                })
                .collect::<Vec<_>>()
        };

        if let Some(existing) = state.workspaces.iter_mut().find(|item| item.id == workspace_id) {
            *existing = workspace.clone();
        } else {
            state.workspaces.push(workspace.clone());
        }
        state.workspace_files.insert(workspace_id.clone(), path_string.clone());
        state.dirty_workspaces.retain(|id| id != &workspace_id);

        WorkspaceLoadReport {
            workspace_id,
            workspace_name: workspace.name,
            missing_connections,
            missing_devices,
        }
    };
    app.emit("app-state-update", state_handle.lock().unwrap().clone())
        .map_err(|error| error.to_string())?;
    app.emit("workspace-load-report", report.clone())
        .map_err(|error| error.to_string())?;
    Ok(report)
}

#[tauri::command]
pub fn export_user_data(app: AppHandle, destination: String) -> Result<(), String> {
    let store = app
        .try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;
    store
        .export_to(std::path::Path::new(&destination))
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_dirty_workspaces(app: AppHandle) -> Vec<String> {
    app.state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .dirty_workspaces
        .clone()
}

#[tauri::command]
pub fn confirm_close(app: AppHandle) -> Result<(), String> {
    let state_handle = app.state::<Mutex<AppState>>();
    let mut state = state_handle.lock().unwrap();
    state.dirty_workspaces.clear();
    Ok(())
}

#[tauri::command]
pub async fn import_user_data(
    app: AppHandle,
    source: String,
    replace: bool,
) -> Result<(), String> {
    let store = app
        .try_state::<StateStore>()
        .ok_or_else(|| "State store is not initialized".to_string())?;
    store
        .import_from(std::path::Path::new(&source), replace)
        .map_err(|error| error.to_string())?;

    let (connections, workspaces, general, appearance) = store
        .load()
        .map_err(|error| error.to_string())?;
    {
        let state_handle = app.state::<Mutex<AppState>>();
        let mut state = state_handle.lock().unwrap();
        state.connections = connections.clone();
        state.workspaces = workspaces;
        state.general = general;
        state.appearance = appearance;
    }
    crate::mqtt::restore_connections(connections).await;
    let state_handle = app.state::<Mutex<AppState>>();
    let state = state_handle.lock().unwrap().clone();
    app.emit("app-state-update", state)
        .map_err(|error| error.to_string())
}
