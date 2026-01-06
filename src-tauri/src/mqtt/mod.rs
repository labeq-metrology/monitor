use lazy_static::lazy_static;
use rumqttc::{AsyncClient, ClientError, Event, MqttOptions, Packet, Publish, QoS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Url};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
// Based on: https://github.com/kuyoonjo/tauri-plugin-mqtt

struct Mqtt {
    client: AsyncClient,
    connected: bool,
    task: JoinHandle<()>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct EventPayload {
    pub id: String,
    pub event: PayloadEvent,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum PayloadEvent {
    #[serde(rename = "connect")]
    Connect(),
    #[serde(rename = "disconnect")]
    Disconnect(),
    #[serde(rename = "message")]
    Message(MqttPublish),
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MqttPublish {
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
    static ref MQTT_CLIENTS: RwLock<HashMap<String, Mqtt>> = RwLock::new(HashMap::new());
}

#[tauri::command]
pub async fn gateway_connect(
    app: AppHandle,
    url: String,
    username: String,
    password: String,
    conn_id: String,
    test: bool, // no cleanup inside task anymore
) -> Result<bool, String> {
    let mut clients = MQTT_CLIENTS.write().await;

    if let Some(client) = clients.remove(&conn_id) {
        client.task.abort();
    }

    let parsed_url = Url::parse(&url).map_err(|e| e.to_string())?;

    let use_tls = parsed_url.scheme() == "mqtts";

    let host = parsed_url
        .host_str()
        .ok_or_else(|| "Invalid host".to_string())?;

    let port = parsed_url.port().unwrap_or(if use_tls { 8883 } else { 1883 });

    let mut options = MqttOptions::new(&conn_id, host, port);

    if !username.is_empty() {
        options.set_credentials(&username, &password);
    }

    options.set_keep_alive(Duration::from_secs(10));

    let (client, mut event_loop) = AsyncClient::new(options, 10);

    client
        .subscribe("#", QoS::AtLeastOnce)
        .await
        .map_err(|e| e.to_string())?;

    let app_handle = app.clone();
    let conn_id_clone = conn_id.clone();

    let task = tokio::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    let _ = app_handle.emit(
                        "mqtt-connected",
                        EventPayload {
                            id: conn_id_clone.clone(),
                            event: PayloadEvent::Connect(),
                        },
                    );
                }
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let _ = app_handle.emit(
                        "mqtt-publish",
                        EventPayload {
                            id: conn_id_clone.clone(),
                            event: PayloadEvent::Message(MqttPublish::from(p)),
                        },
                    );
                }
                Err(e) => {
                    eprintln!("MQTT Error: {e}");
                    let _ = app_handle.emit(
                        "mqtt-disconnected",
                        EventPayload {
                            id: conn_id_clone.clone(),
                            event: PayloadEvent::Disconnect(),
                        },
                    );
                    if test {
                        let mut map = MQTT_CLIENTS.write().await;
                        if let Some(client) = map.remove(&conn_id_clone) {
                            client.task.abort();
                        }
                        break;
                    }
                }
                other => {
                    println!("Other event: {other:?}");
                }
            }
        }
    });

    clients.insert(
        conn_id,
        Mqtt {
            client,
            connected: true,
            task,
        },
    );

    Ok(true)
}

#[tauri::command]
pub async fn gateway_disconnect(app_handle: AppHandle, conn_id: String) -> Result<(), String> {
    if let Some(client) = MQTT_CLIENTS.write().await.remove(&conn_id) {
        client.task.abort();
        let _ = app_handle.emit(
            "mqtt-disconnected",
            EventPayload {
                id: conn_id.clone(),
                event: PayloadEvent::Disconnect(),
            },
        );
        Ok(())
    } else {
        Err("Invalid connection id")?
    }
}

#[tauri::command]
pub async fn gateway_publish(
    conn_id: String,
    topic: String,
    payload: String,
    retain: bool,
) -> Result<(), String> {
    if let Some(client) = MQTT_CLIENTS.read().await.get(&conn_id) {
        match client
            .client
            .publish(topic.clone(), QoS::AtLeastOnce, retain, payload)
            .await
        {
            Ok(..) => Ok(()),
            Err(ClientError::Request(..)) => Err("Invalid topic!".to_string()),
            Err(ClientError::TryRequest(_)) => Err("Invalid topic!".to_string()),
        }
    } else {
        Err("Invalid connection id!".to_string())
    }
}

#[tauri::command]
pub async fn check_connection(conn_id: String) -> bool {
    MQTT_CLIENTS.read().await.contains_key(&conn_id)
}
