use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use crate::state::{update_state, AppState};
use crate::state::connection::{ConnectionData, LabeqDevice};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LabeqWorkspace {
    pub id: String,
    pub name: String,
    pub connections: Folder<ConnectionData>,
    pub devices: Folder<LabeqDevice>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Folder<T> {
    pub items: Vec<Item<T>>,
}

impl <T> Default for Folder<T> {
    fn default() -> Self {
        Self { items: vec![] }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Item<T> {
    Item(T),
    Folder(NestedFolder<T>),
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct NestedFolder<T> {
    pub id: String,
    pub name: String,
    pub items: Vec<Item<T>>,
}

#[tauri::command]
pub fn create_workspace(app: AppHandle, name: String) {
    let workspace = LabeqWorkspace {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        ..Default::default()
    };

    update_state(&app, |state: &mut AppState| {
        state.workspaces.push(workspace);
    });
}

#[tauri::command]
pub fn delete_workspace(app: AppHandle, workspace_id: String) {
    update_state(&app, |state: &mut AppState| {
        state.workspaces.retain(|w| w.id != workspace_id);
    });
}

#[tauri::command]
pub fn rename_workspace(app: AppHandle, workspace_id: String, new_name: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            workspace.name = new_name;
        }
    });
}

#[tauri::command]
pub fn add_device_to_workspace(app: AppHandle, workspace_id: String, device: LabeqDevice) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            workspace.devices.items.push(Item::Item(device));
        }
    });
}

#[tauri::command]
pub fn add_device_folder(app: AppHandle, workspace_id: String, folder_name: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            workspace.devices.items.push(Item::Folder(NestedFolder {
                id: uuid::Uuid::new_v4().to_string(),
                name: folder_name,
                items: vec![],
            }));
        }
    });
}

#[tauri::command]
pub fn remove_device_from_workspace(app: AppHandle, workspace_id: String, device_uuid: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_item_recursive(&mut workspace.devices.items, &device_uuid);
        }
    });
}

#[tauri::command]
pub fn add_connection_to_workspace(app: AppHandle, workspace_id: String, connection: ConnectionData) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            workspace.connections.items.push(Item::Item(connection));
        }
    });
}

#[tauri::command]
pub fn remove_connection_from_workspace(app: AppHandle, workspace_id: String, connection_id: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_item_recursive(&mut workspace.connections.items, &connection_id);
        }
    });
}

fn remove_item_recursive<T>(items: &mut Vec<Item<T>>, id: &str)
where
    T: HasId,
{
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            remove_item_recursive(&mut folder.items, id);
        }
    }

    items.retain(|item| match item {
        Item::Item(data) => data.id() != id,
        Item::Folder(_) => true,
    });
}

// Trait to get ID from different types
trait HasId {
    fn id(&self) -> &str;
}

impl HasId for LabeqDevice {
    fn id(&self) -> &str {
        &self.uuid
    }
}

impl HasId for ConnectionData {
    fn id(&self) -> &str {
        &self.id
    }
}
