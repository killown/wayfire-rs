use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgTemplate {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputDevice {
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionValueResponse {
    pub default: String,
    pub result: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WayfireConfiguration {
    #[serde(rename = "api-version")]
    pub api_version: u32,
    #[serde(rename = "build-branch")]
    pub build_branch: String,
    #[serde(rename = "build-commit")]
    pub build_commit: String,
    #[serde(rename = "plugin-path")]
    pub plugin_path: String,
    #[serde(rename = "plugin-xml-dir")]
    pub plugin_xml_dir: String,
    #[serde(rename = "xwayland-support")]
    pub xwayland_support: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct View {
    pub activated: bool,
    #[serde(rename = "app-id")]
    pub app_id: String,
    #[serde(rename = "base-geometry")]
    pub base_geometry: Geometry,
    pub bbox: Geometry,
    pub focusable: bool,
    pub fullscreen: bool,
    pub geometry: Geometry,
    pub id: i64,
    #[serde(rename = "last-focus-timestamp")]
    pub last_focus_timestamp: i64,
    pub layer: String,
    pub mapped: bool,
    #[serde(rename = "max-size")]
    pub max_size: Size,
    #[serde(rename = "min-size")]
    pub min_size: Size,
    pub minimized: bool,
    #[serde(rename = "output-id")]
    pub output_id: i64,
    #[serde(rename = "output-name")]
    pub output_name: String,
    pub parent: i64,
    pub pid: i64,
    pub role: String,
    pub sticky: bool,
    #[serde(rename = "tiled-edges")]
    pub tiled_edges: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "wset-index")]
    pub wset_index: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewAlpha {
    pub alpha: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub geometry: Geometry,
    pub percent: f64,
    #[serde(rename = "vertical-split")]
    pub vertical_split: Vec<Layout>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub height: i64,
    pub width: i64,
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub height: i64,
    pub width: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetWorkspace {
    pub workspace: Workspace,
    pub workspace_set: Option<WorkspaceSet>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub geometry: Geometry,
    pub id: i64,
    pub name: String,
    #[serde(rename = "workarea")]
    pub work_area: Geometry,
    #[serde(rename = "workspace")]
    pub workspace: Workspace,
    #[serde(rename = "wset-index")]
    pub wset_index: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    #[serde(rename = "grid_height")]
    pub grid_height: i64,
    #[serde(rename = "grid_width")]
    pub grid_width: i64,
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSet {
    #[serde(rename = "index")]
    pub index: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "output-id")]
    pub output_id: i64,
    #[serde(rename = "output-name")]
    pub output_name: String,
    #[serde(rename = "workspace")]
    pub workspace: Workspace,
}
