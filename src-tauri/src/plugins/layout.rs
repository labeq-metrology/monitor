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
pub enum LayoutPanelKind {
    #[serde(rename = "text")]
    TEXT,
    #[serde(rename = "number")]
    NUMBER,
    #[serde(rename = "chart")]
    CHART,
    #[serde(rename = "image")]
    IMAGE,
    #[serde(rename = "custom")]
    CUSTOM,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutPanel {
    kind: LayoutPanelKind,
    parameter: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    readonly: bool,
}
