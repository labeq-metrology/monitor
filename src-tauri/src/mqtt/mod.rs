mod handler;

use crate::state::connection::{ConnectionState, LabeqDevice, ParameterInfo};
use crate::state::{connection::ConnectionData, get_state, update_state, AppState};
use lazy_static::lazy_static;
use rumqttc::{AsyncClient, ClientError, Event, MqttOptions, Packet, Publish, QoS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Url};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use crate::mqtt::handler::handle_publish;

struct ConnectionHandle {
    client: Option<AsyncClient>,
    task: Option<JoinHandle<()>>,
    connection: ConnectionData,
    connected: RwLock<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventPayload {
    pub id: String,
    pub event: PayloadEvent,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PayloadEvent {
    Connect,
    Disconnect,
    Message(MqttPublish),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MqttPublish {
    pub dup: bool,
    pub qos: u8,
    pub retain: bool,
    pub topic: String,
    pub pkid: u16,
    pub payload: Vec<u8>,
}

impl From<Publish> for MqttPublish {
    fn from(p: Publish) -> Self {
        Self {
            dup: p.dup,
            qos: match p.qos {
                QoS::AtMostOnce => 0,
                QoS::AtLeastOnce => 1,
                QoS::ExactlyOnce => 2,
            },
            retain: p.retain,
            topic: p.topic,
            pkid: p.pkid,
            payload: p.payload.to_vec(),
        }
    }
}

lazy_static! {
    static ref MQTT_CLIENTS: RwLock<HashMap<String, ConnectionHandle>> =
        RwLock::new(HashMap::new());
}

#[tauri::command]
pub async fn test_connection(
    url: String,
    username: String,
    password: String,
) -> Result<bool, String> {
    let parsed_url = Url::parse(&url).map_err(|e| format!("Invalid URL: {e}"))?;
    let use_tls = parsed_url.scheme() == "mqtts";
    let host = parsed_url
        .host_str()
        .ok_or_else(|| "Invalid host".to_string())?;
    let port = parsed_url
        .port()
        .unwrap_or(if use_tls { 8883 } else { 1883 });

    let test_id = format!("test_{}", uuid::Uuid::new_v4());
    let mut options = MqttOptions::new(&test_id, host, port);

    if !username.is_empty() {
        options.set_credentials(&username, &password);
    }
    options.set_keep_alive(Duration::from_secs(5));

    let (client, mut event_loop) = AsyncClient::new(options, 10);

    let timeout = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match event_loop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(ack))) => {
                    return if ack.code == rumqttc::ConnectReturnCode::Success {
                        let _ = client.disconnect().await;
                        Ok(true)
                    } else {
                        Err(format!("Connection refused: {:?}", ack.code))
                    }
                }
                Err(e) => {
                    return Err(format!("Connection failed: {e}"));
                }
                _ => {}
            }
        }
    })
    .await;

    timeout.unwrap_or_else(|_| Err("Connection timeout".to_string()))
}

#[tauri::command]
pub async fn create_connection(
    app: AppHandle,
    url: String,
    api_url: String,
    username: String,
    password: String,
    conn_id: String,
    conn_name: String,
) -> Result<(), String> {
    let data = ConnectionData {
        id: conn_id.clone(),
        name: conn_name,
        mqtt_url: url,
        api_url,
        username,
        password,
        state: ConnectionState::Disconnected,
        devices: Vec::new(),
    };

    MQTT_CLIENTS.write().await.insert(
        conn_id.clone(),
        ConnectionHandle {
            client: None,
            task: None,
            connection: data,
            connected: RwLock::new(false),
        },
    );

    let connections = list_connections().await;
    update_state(&app, |state: &mut AppState| {
        state.connections = connections;
    });

    Ok(())
}

#[tauri::command]
pub async fn connection_activate(app: AppHandle, conn_id: String) -> Result<(), String> {
    let _ = internal_deactivate(&app, &conn_id).await;

    let connection_data = {
        let clients = MQTT_CLIENTS.read().await;
        clients
            .get(&conn_id)
            .ok_or_else(|| "Connection not found".to_string())?
            .connection
            .clone()
    };

    let parsed_url =
        Url::parse(&connection_data.mqtt_url).map_err(|e| format!("Invalid URL: {e}"))?;
    let use_tls = parsed_url.scheme() == "mqtts";
    let host = parsed_url
        .host_str()
        .ok_or_else(|| "Invalid host".to_string())?;
    let port = parsed_url
        .port()
        .unwrap_or(if use_tls { 8883 } else { 1883 });

    let mut options = MqttOptions::new(&connection_data.id, host, port);
    if !connection_data.username.is_empty() {
        options.set_credentials(&connection_data.username, &connection_data.password);
    }
    options.set_keep_alive(Duration::from_secs(10));

    let (client, event_loop) = AsyncClient::new(options, 10);

    client
        .subscribe("device/#", QoS::AtLeastOnce)
        .await
        .map_err(|e| format!("Subscription failed: {e}"))?;

    let task = spawn_event_loop(app.clone(), conn_id.clone(), event_loop);

    {
        let mut clients = MQTT_CLIENTS.write().await;
        if let Some(handle) = clients.get_mut(&conn_id) {
            handle.client = Some(client);
            handle.task = Some(task);
            handle.connection.state = ConnectionState::Connecting;
        }
    }

    let connections = list_connections().await;
    update_state(&app, |state: &mut AppState| {
        state.connections = connections;
    });

    _ = update_devices(app, &conn_id).await;

    Ok(())
}

#[tauri::command]
pub async fn connection_deactivate(app: AppHandle, conn_id: String) -> Result<(), String> {
    internal_deactivate(&app, &conn_id).await
}

async fn internal_deactivate(app: &AppHandle, conn_id: &str) -> Result<(), String> {
    {
        // Internal block for RwLock release
        let mut clients = MQTT_CLIENTS.write().await;
        if let Some(handle) = clients.get_mut(conn_id) {
            if let Some(task) = handle.task.take() {
                task.abort();
            }
            handle.client = None;
        }
    }
    update_connection_state(app, &conn_id, false, false).await;

    Ok(())
}

#[tauri::command]
pub async fn connection_remove(app: AppHandle, conn_id: String) -> Result<(), String> {
    let _ = internal_deactivate(&app, &conn_id).await;

    MQTT_CLIENTS.write().await.remove(&conn_id);

    let connections = list_connections().await;
    update_state(&app, |state: &mut AppState| {
        state.connections = connections;
    });

    Ok(())
}

#[tauri::command]
pub async fn gateway_publish(
    conn_id: String,
    topic: String,
    payload: String,
    retain: bool,
) -> Result<(), String> {
    let (client, is_connected) = {
        let clients = MQTT_CLIENTS.read().await;
        let connection = clients
            .get(&conn_id)
            .ok_or_else(|| "Connection not found".to_string())?;

        let is_connected = *connection.connected.read().await;
        let client = connection
            .client
            .clone()
            .ok_or_else(|| "Connection not active".to_string())?;

        (client, is_connected)
    };

    if !is_connected {
        return Err("Not connected to broker".to_string());
    }

    client
        .publish(topic, QoS::AtLeastOnce, retain, payload)
        .await
        .map_err(|e| match e {
            ClientError::Request(_) | ClientError::TryRequest(_) => {
                "Failed to publish message".to_string()
            }
        })
}

#[tauri::command]
pub async fn check_connection(conn_id: String) -> bool {
    if let Some(handle) = MQTT_CLIENTS.read().await.get(&conn_id) {
        *handle.connected.read().await
    } else {
        false
    }
}

#[tauri::command]
pub async fn get_connection_info(conn_id: String) -> Result<ConnectionData, String> {
    let clients = MQTT_CLIENTS.read().await;
    let handle = clients
        .get(&conn_id)
        .ok_or_else(|| "Connection not found".to_string())?;

    Ok(handle.connection.clone())
}

pub async fn list_connections() -> Vec<ConnectionData> {
    let clients = MQTT_CLIENTS.read().await;
    let mut connections = Vec::new();

    for (_, handle) in clients.iter() {
        connections.push(handle.connection.clone());
    }

    connections
}

async fn update_connection_state(app: &AppHandle, conn_id: &str, connected: bool, error: bool) {
    let connections = {
        let mut clients = MQTT_CLIENTS.write().await;

        if let Some(handle) = clients.get_mut(conn_id) {
            *handle.connected.write().await = connected;
            handle.connection.state = if connected {
                ConnectionState::Connected
            } else if error {
                ConnectionState::ConnectionError
            } else {
                ConnectionState::Disconnected
            };
        }

        clients.values().map(|c| c.connection.clone()).collect()
    };

    update_state(app, |state| {
        state.connections = connections;
    });
}

#[tauri::command]
pub async fn update_devices(app: AppHandle, conn_id: &str) -> Result<Vec<LabeqDevice>, ()> {
    let api_url = MQTT_CLIENTS
        .read()
        .await
        .get(conn_id)
        .ok_or(())?
        .connection
        .api_url
        .clone();

    let devices = reqwest::get(format!("{}/devices", api_url))
        .await
        .map_err(|_| ())?
        .json::<Vec<LabeqDevice>>()
        .await
        .map_err(|_| ())?;

    let connections = {
        let mut clients = MQTT_CLIENTS.write().await;
        clients.get_mut(conn_id).ok_or(())?.connection.devices = devices.clone();

        clients.values().map(|c| c.connection.clone()).collect()
    };

    update_state(&app, |state| {
        state.connections = connections;
    });

    Ok(devices)
}

#[tauri::command]
pub async fn update_device_parameters(
    app: AppHandle,
    conn_id: &str,
    device_id: &str,
) -> Result<LabeqDevice, ()> {
    let connection: Option<ConnectionData> = get_state(app.clone())
        .connections
        .iter()
        .find(|c| c.id == conn_id)
        .cloned();

    if let Some(connection) = connection {
        let api_url = connection.api_url;

        let device_with_params = reqwest::get(format!("{}/devices/{id}", api_url, id = device_id))
            .await
            .map_err(|_| ())?
            .json::<LabeqDevice>()
            .await
            .map_err(|_| ())?;

        let connections = {
            let mut clients = MQTT_CLIENTS.write().await;
            let connection = clients.get_mut(conn_id).ok_or(())?;

            if let Some(device) = connection
                .connection
                .devices
                .iter_mut()
                .find(|d| d.uuid == device_id)
            {
                *device = device_with_params.clone();
            }

            clients.values().map(|c| c.connection.clone()).collect()
        };

        update_state(&app, |state| {
            state.connections = connections;
        });

        return Ok(device_with_params)
    }
    Err(())
}

fn spawn_event_loop(
    app: AppHandle,
    conn_id: String,
    mut event_loop: rumqttc::EventLoop,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    update_connection_state(&app, &conn_id, true, false).await;
                    emit_event(&app, &conn_id, PayloadEvent::Connect);
                }
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let packet = MqttPublish::from(p);
                    emit_event(&app, &conn_id, PayloadEvent::Message(packet.clone()));
                    handle_publish(&app, &conn_id, packet)
                }
                Err(e) => {
                    eprintln!("MQTT connection error [{conn_id}]: [{e}]");
                    update_connection_state(&app, &conn_id, false, true).await;
                    emit_event(&app, &conn_id, PayloadEvent::Disconnect);

                    let _ = internal_deactivate(&app, &conn_id).await;
                    break;
                }
                _ => {}
            }
        }
    })
}

fn emit_event(app: &AppHandle, conn_id: &str, event: PayloadEvent) {
    let event_name = match &event {
        PayloadEvent::Connect => "mqtt-connected",
        PayloadEvent::Disconnect => "mqtt-disconnected",
        PayloadEvent::Message(_) => "mqtt-publish",
    };

    let payload = EventPayload {
        id: conn_id.to_string(),
        event,
    };

    if let Err(e) = app.emit(event_name, payload) {
        eprintln!("Failed to emit event {}: {}", event_name, e);
    }
}
