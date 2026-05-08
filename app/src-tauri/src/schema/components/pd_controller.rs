use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PdControllerConfig {
    pub kp: f64,
    pub kd: f64,
    pub target_angle_rad: f64,
}
