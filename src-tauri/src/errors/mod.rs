use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct LabeqError {
    pub message: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum LabeqErrors {
    PluginError(LabeqError)
}
