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
    Text,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "number-unit")]
    NumberUnit,
    #[serde(rename = "chart")]
    Chart,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutPanel {
    kind: LayoutPanelKind,
    parameter: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    readonly: bool,
}
