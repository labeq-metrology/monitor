use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    ConnectionError,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionData {
    pub id: String,
    pub mqtt_url: String,
    pub api_url: String,
    pub username: String,
    pub password: String,
    pub state: ConnectionState,
    pub devices: Vec<LabeqDevice>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabeqDevice {
    pub uuid: String,
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub driver: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub connection_ref: String,
}

#[tauri::command]
pub async fn list_connections(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<ConnectionData>, ()> {
    Ok(state.lock().unwrap().connections.clone())
}
