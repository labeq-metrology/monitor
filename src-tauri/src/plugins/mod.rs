use crate::errors::{LabeqError, LabeqErrors};
use extism::convert::Json;
use extism::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

lazy_static! {
    static ref LOADED_PLUGINS: RwLock<HashMap<String, Plugin>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Deserialize)]
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
    plugins_lock.clear();
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
                let manifest =
                    loaded_plugin.call::<(), Json<LabeqPluginManifest>>("loadManifest", ());

                if (manifest.is_err()) {
                    app.emit(
                        "plugin-error",
                        LabeqErrors::PluginError(LabeqError {
                            message: format!(
                                "Unable to load plugin manifest ({})",
                                plugin.path().canonicalize().unwrap().display()
                            ),
                            code: "LQ-MN-P-02".to_string(),
                        }),
                    )
                    .unwrap();
                    continue;
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
pub fn load_layout(driver: String, variant: String) -> Result<String, String> {
    Ok("{}".to_string())
}
