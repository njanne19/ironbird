use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;

/// Describes the serializable structure of 
/// a project (ibd) file. 
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IronbirdProject {
    pub schema_version: String,
    pub project_name: String,
    pub components: Vec<IronbirdComponent>,
    pub wiring: Vec<Wire>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IronbirdComponent {
    pub id: String,
    pub component_type: String,
    pub ports: Ports,
    pub config: HashMap<String, Value>,
    pub initial: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Ports {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}
