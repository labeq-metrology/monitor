pub(crate) mod layout;

use crate::errors::{LabeqError, LabeqErrors};
use crate::plugins::layout::LayoutSchema;
use extism::convert::Json;
use extism::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

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

/// Helper function to get plugin by driver name
async fn get_plugin_by_driver(driver: &str) -> Result<String, String> {
    let drivers_lock = SUPPORTED_DRIVERS.read().await;
    drivers_lock
        .get(driver)
        .cloned()
        .ok_or_else(|| format!("Driver {} not supported", driver))
}

/// Helper function to emit plugin error
fn emit_plugin_error(app: &AppHandle, message: String, code: &str) {
    let _ = app.emit(
        "plugin-error",
        LabeqErrors::PluginError(LabeqError { message, code: code.to_string() }),
    );
}

/// Helper function to get plugin by driver name and acquire write lock
async fn get_plugin_mut(driver: &str) -> Result<(String, tokio::sync::RwLockWriteGuard<'static, HashMap<String, Plugin>>), String> {
    let plugin_id = get_plugin_by_driver(driver).await?;
    let plugins_lock = LOADED_PLUGINS.write().await;
    Ok((plugin_id, plugins_lock))
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

        if plugin.file_type().unwrap().is_dir()
            || !plugin.path().extension().unwrap().eq("wasm") {
            continue;
        }

        match load_plugin(plugin.path()).await {
            Ok(mut loaded_plugin) => {
                match loaded_plugin.call::<(), Json<LabeqPluginManifest>>("loadManifest", ()) {
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

                        emit_plugin_error(
                            &app,
                            format!("Unable to load plugin manifest ({})", path),
                            "LQ-MN-P-02",
                        );
                    }
                }
            }
            Err(err) => {
                emit_plugin_error(&app, err, "LQ-MN-P-01");
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn load_plugin(path: PathBuf) -> Result<Plugin, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let wasm_file = Wasm::data(data);
    let manifest = Manifest::new([wasm_file]);
    Plugin::new(&manifest, [], true).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_plugins() -> Result<Vec<String>, String> {
    let manifest_lock = LOADED_MANIFESTS.read().await;
    Ok(manifest_lock
        .values()
        .map(|manifest| serde_json::to_string(manifest).unwrap())
        .collect())
}

#[tauri::command]
pub async fn load_layout(
    app: AppHandle,
    driver: String,
    variant: String,
) -> Result<LayoutSchema, String> {
    let (plugin_id, mut plugins_lock) = match get_plugin_mut(&driver).await {
        Ok(result) => result,
        Err(err) => {
            emit_plugin_error(&app, err.clone(), "LQ-MN-P-04");
            return Err(err);
        }
    };

    let result = plugins_lock
        .get_mut(&plugin_id)
        .ok_or_else(|| format!("Plugin {} not found", plugin_id))?
        .call::<String, Json<LayoutSchema>>("loadLayout", format!("{},{}", driver, variant))
        .map(|Json(layout)| layout)
        .map_err(|e| e.to_string());

    match result {
        Ok(layout) => Ok(layout),
        Err(err) => {
            emit_plugin_error(&app, format!("Unable to load layout ({})", driver), "LQ-MN-P-03");
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn get_panel_information(
    app: AppHandle,
    driver: String,
    variant: String,
    parameter: String,
) -> Result<String, String> {
    let (plugin_id, mut plugins_lock) = match get_plugin_mut(&driver).await {
        Ok(result) => result,
        Err(err) => {
            emit_plugin_error(&app, err.clone(), "LQ-MN-P-04");
            return Err(err);
        }
    };

    plugins_lock
        .get_mut(&plugin_id)
        .ok_or_else(|| format!("Plugin {} not found", plugin_id))?
        .call::<String, String>("getPanelInformation", format!("{},{},{}", driver, variant, parameter))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn format_param_to_panel(
    app: AppHandle,
    driver: String,
    variant: String,
    parameter: String,
    value: String,
) -> Result<String, String> {
    let (plugin_id, mut plugins_lock) = match get_plugin_mut(&driver).await {
        Ok(result) => result,
        Err(err) => {
            emit_plugin_error(&app, err.clone(), "LQ-MN-P-04");
            return Err(err);
        }
    };

    plugins_lock
        .get_mut(&plugin_id)
        .ok_or_else(|| format!("Plugin {} not found", plugin_id))?
        .call::<String, String>("formatParameterToPanel", format!("{},{},{}~{}", driver, variant, parameter, value))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn format_panel_to_param(
    app: AppHandle,
    driver: String,
    variant: String,
    parameter: String,
    value: String,
) -> Result<String, String> {
    let (plugin_id, mut plugins_lock) = match get_plugin_mut(&driver).await {
        Ok(result) => result,
        Err(err) => {
            emit_plugin_error(&app, err.clone(), "LQ-MN-P-04");
            return Err(err);
        }
    };

    plugins_lock
        .get_mut(&plugin_id)
        .ok_or_else(|| format!("Plugin {} not found", plugin_id))?
        .call::<String, String>("formatPanelToParameter", format!("{},{},{}~{}", driver, variant, parameter, value))
        .map_err(|e| e.to_string())
}