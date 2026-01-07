use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct AppState {
    pub connections: String,
}
