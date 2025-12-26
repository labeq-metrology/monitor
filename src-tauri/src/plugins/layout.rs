use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutSchema {
    rows: Vec<LayoutRow>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutRow {
    #[serde(rename = "groupName")]
    group_name: Option<String>,
    columns: Vec<LayoutColumn>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutColumn {
    panels: Vec<LayoutPanel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutPanel {
    kind: String,
    parameter: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    readonly: bool,
}
