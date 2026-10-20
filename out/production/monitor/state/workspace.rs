use crate::state::{update_state, AppState};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LabeqWorkspace {
    pub id: String,
    pub name: String,
    pub connections: Folder<String>,
    pub devices: Folder<(String, String)>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Folder<T> {
    pub items: Vec<Item<T>>,
}

impl<T> Default for Folder<T> {
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

#[tauri::command]
pub fn create_workspace(app: AppHandle, name: String) -> String {
    let id = uuid::Uuid::new_v4().to_string();
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
pub fn add_device_to_workspace(
    app: AppHandle,
    workspace_id: String,
    connection_ref: String,
    uuid: String,
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            if (!workspace.devices.items.iter().any(
                |item| matches!(item, Item::Item((c, u)) if c == &connection_ref && u == &uuid),
            )) {
                workspace
                    .devices
                    .items
                    .push(Item::Item((connection_ref, uuid)));
            }
        }
    });
}

#[tauri::command]
pub fn remove_device_from_workspace(app: AppHandle, workspace_id: String, device_id: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_item_recursive(&mut workspace.devices.items, |(_conn, uuid)| {
                uuid == &device_id
            });
        }
    });
}

#[tauri::command]
pub fn create_device_folder(
    app: AppHandle,
    workspace_id: String,
    parent_id: Option<String>,
    folder_name: String,
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let new_folder = NestedFolder {
                id: uuid::Uuid::new_v4().to_string(),
                name: folder_name,
                items: vec![],
            };
            match parent_id {
                Some(pid) => add_folder_to_parent(&mut workspace.devices.items, &pid, new_folder),
                None => workspace.devices.items.push(Item::Folder(new_folder)),
            }
        }
    });
}

#[tauri::command]
pub fn rename_device_folder(
    app: AppHandle,
    workspace_id: String,
    folder_id: String,
    new_name: String,
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
    folder_id: Option<String>,
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let device = find_and_remove(&mut workspace.devices.items, |(_conn, uuid)| {
                uuid == &device_id
            });
            if let Some(device) = device {
                match folder_id {
                    Some(fid) => add_item_to_folder(&mut workspace.devices.items, &fid, device),
                    None => workspace.devices.items.push(Item::Item(device)),
                }
            }
        }
    });
}

#[tauri::command]
pub fn add_connection_to_workspace(app: AppHandle, workspace_id: String, connection_id: String) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            if (!workspace
                .connections
                .items
                .iter()
                .any(|item| matches!(item, Item::Item(c) if c == &connection_id)))
            {
                workspace.connections.items.push(Item::Item(connection_id));
            }
        }
    });
}

#[tauri::command]
pub fn remove_connection_from_workspace(
    app: AppHandle,
    workspace_id: String,
    connection_id: String,
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            remove_item_recursive(&mut workspace.connections.items, |id| id == &connection_id);
        }
    });
}

#[tauri::command]
pub fn create_connection_folder(
    app: AppHandle,
    workspace_id: String,
    parent_id: Option<String>,
    folder_name: String,
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let new_folder = NestedFolder {
                id: uuid::Uuid::new_v4().to_string(),
                name: folder_name,
                items: vec![],
            };
            match parent_id {
                Some(pid) => {
                    add_folder_to_parent(&mut workspace.connections.items, &pid, new_folder)
                }
                None => workspace.connections.items.push(Item::Folder(new_folder)),
            }
        }
    });
}

#[tauri::command]
pub fn rename_connection_folder(
    app: AppHandle,
    workspace_id: String,
    folder_id: String,
    new_name: String,
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
    folder_id: Option<String>,
) {
    update_state(&app, |state: &mut AppState| {
        if let Some(workspace) = state.workspaces.iter_mut().find(|w| w.id == workspace_id) {
            let connection =
                find_and_remove(&mut workspace.connections.items, |id| id == &connection_id);
            if let Some(connection) = connection {
                match folder_id {
                    Some(fid) => {
                        add_item_to_folder(&mut workspace.connections.items, &fid, connection)
                    }
                    None => workspace.connections.items.push(Item::Item(connection)),
                }
            }
        }
    });
}

// Remove all items matching `predicate` anywhere in the tree.
fn remove_item_recursive<T, F>(items: &mut Vec<Item<T>>, predicate: F)
where
    F: Fn(&T) -> bool + Copy,
{
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            remove_item_recursive(&mut folder.items, predicate);
        }
    }
    items.retain(|item| match item {
        Item::Item(data) => !predicate(data),
        Item::Folder(_) => true,
    });
}

// Remove a folder by ID anywhere in the tree.
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

// Rename a folder by ID anywhere in the tree.
fn rename_folder_recursive<T>(items: &mut Vec<Item<T>>, folder_id: &str, new_name: String) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if folder.id == folder_id {
                folder.name = new_name;
                return;
            }
            rename_folder_recursive(&mut folder.items, folder_id, new_name.clone());
        }
    }
}

// Add a new sub-folder inside the folder matching `parent_id`.
fn add_folder_to_parent<T: Clone>(
    items: &mut Vec<Item<T>>,
    parent_id: &str,
    new_folder: NestedFolder<T>,
) {
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

// Find and remove the first item matching `predicate`, returning it.
fn find_and_remove<T, F>(items: &mut Vec<Item<T>>, predicate: F) -> Option<T>
where
    F: Fn(&T) -> bool + Copy,
{
    if let Some(index) = items
        .iter()
        .position(|item| matches!(item, Item::Item(v) if predicate(v)))
    {
        if let Item::Item(value) = items.remove(index) {
            return Some(value);
        }
    }
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if let Some(found) = find_and_remove(&mut folder.items, predicate) {
                return Some(found);
            }
        }
    }
    None
}

// Add an item inside the folder matching `folder_id`.
fn add_item_to_folder<T: Clone>(items: &mut Vec<Item<T>>, folder_id: &str, value: T) {
    for item in items.iter_mut() {
        if let Item::Folder(folder) = item {
            if folder.id == folder_id {
                folder.items.push(Item::Item(value));
                return;
            }
            add_item_to_folder(&mut folder.items, folder_id, value.clone());
        }
    }
}
