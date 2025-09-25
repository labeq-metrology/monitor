pub(crate) mod layout;

use crate::errors::{LabeqError, LabeqErrors};
use extism::convert::Json;
use extism::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use crate::plugins::layout::LayoutSchema;

lazy_static! {
    static ref LOADED_PLUGINS: RwLock<HashMap<String, Plugin>> = RwLock::new(HashMap::new());
    static ref LOADED_MANIFESTS: RwLock<HashMap<String, LabeqPluginManifest>> =
        RwLock::new(HashMap::new());
    static ref SUPPORTED_DRIVERS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabeqPluginManifest {
    pub name: String,
    pub version: String,
    pub id: String,
    pub drivers: Vec<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[tauri::command]
pub async fn init_plugins(app: AppHandle) -> Result<(), ()> {
    if !fs::exists("plugins").unwrap() {
        fs::create_dir("plugins").unwrap();
    }

    let plugins = fs::read_dir("./plugins").unwrap();
    let mut plugins_lock = LOADED_PLUGINS.write().await;
    let mut drivers_lock = SUPPORTED_DRIVERS.write().await;
    let mut manifests_lock = LOADED_MANIFESTS.write().await;
    plugins_lock.clear();
    drivers_lock.clear();
    for plugin in plugins {
        let plugin = plugin.unwrap();
        if plugin.file_type().unwrap().is_dir() {
            continue;
        }
        if !plugin.path().extension().unwrap().eq("wasm") {
            continue;
        }
        match load_plugin(plugin.path()).await {
            Ok(mut loaded_plugin) => {
                let manifest_result =
                    loaded_plugin.call::<(), Json<LabeqPluginManifest>>("loadManifest", ());

                match manifest_result {
                    Ok(Json(manifest)) => {
                        let plugin_id = manifest.id.clone();

                        plugins_lock.insert(plugin_id.clone(), loaded_plugin);

                        for driver in &manifest.drivers {
                            drivers_lock.insert(driver.clone(), plugin_id.clone());
                        }

                        manifests_lock.insert(plugin_id, manifest);
                    }
                    Err(_) => {
                        let path = plugin
                            .path()
                            .canonicalize()
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|_| "<unknown>".to_string());

                        let error = LabeqErrors::PluginError(LabeqError {
                            message: format!("Unable to load plugin manifest ({})", path),
                            code: "LQ-MN-P-02".to_string(),
                        });

                        app.emit("plugin-error", error).unwrap();
                        continue;
                    }
                }
            }
            Err(err) => {
                app.emit(
                    "plugin-error",
                    LabeqErrors::PluginError(LabeqError {
                        message: err,
                        code: "LQ-MN-P-01".to_string(),
                    }),
                )
                .unwrap();
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn load_plugin(path: PathBuf) -> Result<Plugin, String> {
    match fs::read(path) {
        Ok(data) => {
            let wasm_file = Wasm::data(data);
            let manifest = Manifest::new([wasm_file]);
            match Plugin::new(&manifest, [], true) {
                Ok(plugin) => Ok(plugin),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(msg) => Err(msg.to_string()),
    }
}

#[tauri::command]
pub async fn list_plugins() -> Result<Vec<String>, String> {
    let manifest_lock = LOADED_MANIFESTS.read().await;
    let mut result = Vec::new();
    manifest_lock.iter().for_each(|(_, manifest)| {
        result.push(serde_json::to_string(manifest).unwrap());
    });
    Ok(result)
}

#[tauri::command]
pub async fn load_layout(
    app: AppHandle,
    driver: String,
    variant: String,
) -> Result<LayoutSchema, String> {
    let mut plugins_lock = LOADED_PLUGINS.write().await;
    let drivers_lock = SUPPORTED_DRIVERS.read().await;

    let plugin_id = match drivers_lock.get(&driver) {
        Some(id) => id,
        None => {
            app.emit(
                "plugin-error",
                LabeqErrors::PluginError(LabeqError {
                    message: format!("Driver {} not supported", driver),
                    code: "LQ-MN-P-04".to_string(),
                }),
            )
            .unwrap();
            return Err(format!("Driver {} not supported", driver));
        }
    };

    let plugin = plugins_lock.get_mut(plugin_id).unwrap();

    let layout_result = plugin.call::<String, Json<layout::LayoutSchema>>(
        "loadLayout",
        format!("{},{}", driver, variant),
    );

    match layout_result {
        Ok(Json(layout)) => Ok(layout),
        Err(_) => {
            app.emit(
                "plugin-error",
                LabeqErrors::PluginError(LabeqError {
                    message: format!("Unable to load layout ({})", driver),
                    code: "LQ-MN-P-03".to_string(),
                }),
            )
            .unwrap();
            Err(format!("Unable to load layout ({})", driver))
        }
    }
}
