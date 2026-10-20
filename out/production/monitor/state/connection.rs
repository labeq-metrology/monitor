use std::collections::HashMap;
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
    pub name: String,
    pub mqtt_url: String,
    pub api_url: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub state: ConnectionState,
    pub devices: Vec<LabeqDevice>,
    #[serde(default)]
    pub scripts: Vec<ScriptStatus>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStatus {
    pub execution_id: String,
    pub script: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParameterInfo {
    pub param_type: String,
    pub value: ParameterValue
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "paramType", content = "value")]
pub enum ParameterValue {
    StringValue(String),
    IntegerValue(i64),
    DoubleValue(f64),
    BytesValue(Vec<u8>),
    IntegerArrayValue(Vec<i64>),
    DoubleArrayValue(Vec<f64>)
}

impl Default for ParameterValue {
    fn default() -> Self {
        ParameterValue::StringValue(String::new())
    }
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
    pub parameters: HashMap<String, ParameterValue>
}

#[tauri::command]
pub async fn list_connections(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<ConnectionData>, ()> {
    Ok(state.lock().unwrap().connections.clone())
}
