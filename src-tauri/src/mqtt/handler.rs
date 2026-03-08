use crate::mqtt::MqttPublish;
use crate::state::connection::{ParameterInfo, ParameterValue};
use crate::state::update_state;
use regex::Regex;
use serde::Deserialize;
use tauri::AppHandle;

#[derive(Deserialize)]
struct ParameterUpdate {
    #[serde(rename = "newValue")]
    new_value: ParameterValue,
}

pub fn handle_publish(app: &AppHandle, conn_id: &str, packet: MqttPublish) {
    let topic = packet.topic;

    let topic_regex =
        Regex::new("^device/(?<device>.+)/parameters/(?<parameter>.+)/(?<action>.+)").unwrap();

    let caps = match topic_regex.captures(&topic) {
        Some(c) => c,
        None => return,
    };

    let device = caps.name("device").unwrap().as_str();
    let parameter = caps.name("parameter").unwrap().as_str();
    let action = caps.name("action").unwrap().as_str();

    if action != "update" {
        return;
    }

    update_state(app, |state| {
        if let Some(connection) = state.connections.iter_mut().find(|c| c.id == conn_id) {
            if let Some(device) = connection.devices.iter_mut().find(|d| d.uuid == device) {
                if (device.parameters.contains_key(parameter)) {
                    let payload_str = String::from_utf8(packet.payload).unwrap();
                    let payload = serde_json::from_str::<ParameterUpdate>(&payload_str);

                    match payload {
                        Ok(update) => {
                            device.parameters.insert(parameter.to_string(), update.new_value);
                        }
                        Err(e) => return,
                    }
                }
            }
        }
    });
}
