use crate::state::connection::{ConnectionState, LabeqDevice};
use crate::state::{connection::ConnectionData, connection::ConnectionState::*, update_state, AppState};
use lazy_static::lazy_static;
use rumqttc::{AsyncClient, ClientError, Event, MqttOptions, Packet, Publish, QoS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State, Url};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

struct ConnectionHandle {
    client: AsyncClient,
    task: JoinHandle<()>,
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
pub async fn gateway_connect(
    app: AppHandle,
    url: String,
    api_url: String,
    username: String,
    password: String,
    conn_id: String,
) -> Result<(), String> {
    // Disconnect existing connection if any
    disconnect_internal(&conn_id).await;

    // Parse and validate URL
    let parsed_url = Url::parse(&url).map_err(|e| format!("Invalid URL: {}", e))?;
    let use_tls = parsed_url.scheme() == "mqtts";
    let host = parsed_url
        .host_str()
        .ok_or_else(|| "Invalid host".to_string())?;
    let port = parsed_url
        .port()
        .unwrap_or(if use_tls { 8883 } else { 1883 });

    // Configure MQTT options
    let mut options = MqttOptions::new(&conn_id, host, port);
    if !username.is_empty() {
        options.set_credentials(&username, &password);
    }
    options.set_keep_alive(Duration::from_secs(10));

    // Create client and event loop
    let (client, event_loop) = AsyncClient::new(options, 10);

    client
        .subscribe("device/#", QoS::AtLeastOnce)
        .await
        .map_err(|e| format!("Subscription failed: {}", e))?;

    let task = spawn_event_loop(app.clone(), conn_id.clone(), event_loop);

    let data = ConnectionData {
        id: conn_id.clone(),
        mqtt_url: url,
        api_url,
        username,
        password,
        state: ConnectionState::Connecting,
        devices: Vec::new(),
    };

    MQTT_CLIENTS.write().await.insert(
        conn_id.clone(),
        ConnectionHandle {
            client,
            task,
            connection: data,
            connected: RwLock::new(false),
        },
    );

    let connections = list_connections().await;
    update_state(&app, |state: &mut AppState| {
        state.connections = connections;
    });

    _ = update_devices(app, &conn_id).await;

    Ok(())
}

#[tauri::command]
pub async fn gateway_disconnect(conn_id: String) -> Result<(), String> {
    disconnect_internal(&conn_id).await;
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
        let client = connection.client.clone();

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

async fn disconnect_internal(conn_id: &str) {
    if let Some(handle) = MQTT_CLIENTS.write().await.remove(conn_id) {
        handle.task.abort();
    }
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
        clients
            .get_mut(conn_id)
            .ok_or(())?
            .connection
            .devices = devices.clone();

        clients.values().map(|c| c.connection.clone()).collect()
    };

    update_state(&app, |state| {
        state.connections = connections;
    });

    Ok(devices)
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
                    emit_event(&app, &conn_id, PayloadEvent::Message(MqttPublish::from(p)));
                }
                Err(e) => {
                    eprintln!("MQTT connection error [{conn_id}]: [{e}]");
                    update_connection_state(&app, &conn_id, false, true).await;
                    emit_event(&app, &conn_id, PayloadEvent::Disconnect);

                    disconnect_internal(&conn_id).await;
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
