// src-tauri/src/state/workspace.rs
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
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
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

#[tauri::command]
pub fn create_workspace(app: AppHandle, name: String) -> String {
    let id: String = uuid::Uuid::new_v4().to_string();
    let workspace = LabeqWorkspace {
        id: id.clone(),
        name,
        ..Default::default()
    };

    update_state(&app, |state: &mut AppState| {
        state.workspaces.push(workspace);
    });
    id
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
pub fn remove_device_from_workspace(app: AppHandle, workspace_id: String, device_id: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_item_recursive(&mut workspace.devices.items, &device_id);
        }
    });
}

#[tauri::command]
pub fn create_device_folder(
    app: AppHandle,
    workspace_id: String,
    parent_id: Option<String>,
    folder_name: String
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let new_folder = NestedFolder {
                id: uuid::Uuid::new_v4().to_string(),
                name: folder_name,
                items: vec![],
            };

            if let Some(parent_id) = parent_id {
                // Add to parent folder
                add_folder_to_parent(&mut workspace.devices.items, &parent_id, new_folder);
            } else {
                // Add to root
                workspace.devices.items.push(Item::Folder(new_folder));
            }
        }
    });
}

#[tauri::command]
pub fn rename_device_folder(
    app: AppHandle,
    workspace_id: String,
    folder_id: String,
    new_name: String
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            rename_folder_recursive(&mut workspace.devices.items, &folder_id, new_name);
        }
    });
}

#[tauri::command]
pub fn delete_device_folder(app: AppHandle, workspace_id: String, folder_id: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_folder_recursive(&mut workspace.devices.items, &folder_id);
        }
    });
}

#[tauri::command]
pub fn move_device_to_folder(
    app: AppHandle,
    workspace_id: String,
    device_id: String,
    folder_id: Option<String>
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let device = find_and_remove_device(&mut workspace.devices.items, &device_id);

            if let Some(device) = device {
                if let Some(folder_id) = folder_id {
                    // Add to specific folder
                    add_device_to_folder(&mut workspace.devices.items, &folder_id, device);
                } else {
                    // Add to root
                    workspace.devices.items.push(Item::Item(device));
                }
            }
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

#[tauri::command]
pub fn create_connection_folder(
    app: AppHandle,
    workspace_id: String,
    parent_id: Option<String>,
    folder_name: String
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let new_folder = NestedFolder {
                id: uuid::Uuid::new_v4().to_string(),
                name: folder_name,
                items: vec![],
            };

            if let Some(parent_id) = parent_id {
                add_folder_to_parent(&mut workspace.connections.items, &parent_id, new_folder);
            } else {
                workspace.connections.items.push(Item::Folder(new_folder));
            }
        }
    });
}

#[tauri::command]
pub fn rename_connection_folder(
    app: AppHandle,
    workspace_id: String,
    folder_id: String,
    new_name: String
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            rename_folder_recursive(&mut workspace.connections.items, &folder_id, new_name);
        }
    });
}

#[tauri::command]
pub fn delete_connection_folder(app: AppHandle, workspace_id: String, folder_id: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_folder_recursive(&mut workspace.connections.items, &folder_id);
        }
    });
}

#[tauri::command]
pub fn move_connection_to_folder(
    app: AppHandle,
    workspace_id: String,
    connection_id: String,
    folder_id: Option<String>
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let connection = find_and_remove_connection(&mut workspace.connections.items, &connection_id);

            if let Some(connection) = connection {
                if let Some(folder_id) = folder_id {
                    add_connection_to_folder(&mut workspace.connections.items, &folder_id, connection);
                } else {
                    workspace.connections.items.push(Item::Item(connection));
                }
            }
        }
    });
}

// Remove an item (by ID) recursively from the tree
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

// Remove a folder (by ID) recursively from the tree
fn remove_folder_recursive<T>(items: &mut Vec<Item<T>>, folder_id: &str) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            remove_folder_recursive(&mut folder.items, folder_id);
        }
    }

    items.retain(|item| match item {
        Item::Folder(folder) => folder.id != folder_id,
        Item::Item(_) => true,
    });
}

// Rename a folder (by ID) recursively in the tree
fn rename_folder_recursive<T>(items: &mut Vec<Item<T>>, folder_id: &str, new_name: String) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if folder.id == folder_id {
                folder.name = new_name.clone();
                return;
            }
            rename_folder_recursive(&mut folder.items, folder_id, new_name.clone());
        }
    }
}

// Add a new folder to a parent folder
fn add_folder_to_parent<T: Clone>(items: &mut Vec<Item<T>>, parent_id: &str, new_folder: NestedFolder<T>) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if folder.id == parent_id {
                folder.items.push(Item::Folder(new_folder));
                return;
            }
            add_folder_to_parent(&mut folder.items, parent_id, new_folder.clone());
        }
    }
}

// Find and remove a device from the tree, returning it if found
fn find_and_remove_device(items: &mut Vec<Item<LabeqDevice>>, device_id: &str) -> Option<LabeqDevice> {
    let index_to_remove = items.iter().position(|item| {
        if let Item::Item(device) = item {
            device.id() == device_id
        } else {
            false
        }
    });

    if let Some(index) = index_to_remove {
        if let Item::Item(device) = items.remove(index) {
            return Some(device);
        }
    }

    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if let Some(device) = find_and_remove_device(&mut folder.items, device_id) {
                return Some(device);
            }
        }
    }

    None
}

// Add a device to a specific folder
fn add_device_to_folder(items: &mut Vec<Item<LabeqDevice>>, folder_id: &str, device: LabeqDevice) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if folder.id == folder_id {
                folder.items.push(Item::Item(device));
                return;
            }
            add_device_to_folder(&mut folder.items, folder_id, device.clone());
        }
    }
}

// Find and remove a connection from the tree, returning it if found
fn find_and_remove_connection(items: &mut Vec<Item<ConnectionData>>, connection_id: &str) -> Option<ConnectionData> {
    let found_index = items
        .iter()
        .position(|item| {
            if let Item::Item(connection) = item {
                connection.id() == connection_id
            } else {
                false
            }
        });

    if let Some(i) = found_index {
        if let Item::Item(connection) = items.remove(i) {
            return Some(connection);
        }
    }

    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if let Some(connection) = find_and_remove_connection(&mut folder.items, connection_id) {
                return Some(connection);
            }
        }
    }

    None
}

// Add a connection to a specific folder
fn add_connection_to_folder(items: &mut [Item<ConnectionData>], folder_id: &str, connection: ConnectionData) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if folder.id == folder_id {
                folder.items.push(Item::Item(connection));
                return;
            }
            add_connection_to_folder(&mut folder.items, folder_id, connection.clone());
        }
    }
}