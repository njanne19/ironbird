pub mod components;

use serde::{Deserialize, Serialize};
use specta::Type;
use components::ComponentKind;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IronbirdProject {
    pub schema_version: String,
    pub project_name: String,
    pub components: Vec<Component>,
    pub wiring: Vec<Wire>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Component {
    pub id: String,
    pub ports: Ports,
    #[serde(flatten)]
    pub kind: ComponentKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Ports {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Wire {
    pub from: String,
    pub from_port: String,
    pub to: String,
    pub to_port: String,
}

pub fn parse_ibd_project(json: &str) -> Result<IronbirdProject> {
    let res = serde_json::from_str(json)?;
    Ok(res)
}
