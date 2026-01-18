pub mod connection;

use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::AsyncWriteExt;
use crate::state::connection::ConnectionData;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppState {
    pub connections: Vec<ConnectionData>,
}

pub fn update_state(app: &AppHandle, f: impl FnOnce(&mut AppState)) {
    let state = app.state::<Mutex<AppState>>();
    let state_clone = {
        let mut lock = state.lock().unwrap();
        f(&mut lock);
        lock.clone()
    };

    app.emit("app-state-update", state_clone)
        .expect("Failed to emit state update event");
}

#[tauri::command]
pub fn get_state(app: AppHandle) -> AppState {
    app.state::<Mutex<AppState>>().lock().unwrap().clone()
}
