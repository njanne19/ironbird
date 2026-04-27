use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PendulumConfig {
    pub length_m: f64,
    pub mass_kg: f64,
    pub damping: f64,
    pub gravity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PendulumInitial {
    pub angle_rad: f64,
    pub angular_velocity_rad_s: f64,
}
