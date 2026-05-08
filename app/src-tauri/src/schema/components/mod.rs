pub mod pendulum;
pub mod pd_controller;

use serde::{Deserialize, Serialize};
use specta::Type;
use pendulum::{PendulumConfig, PendulumInitial};
use pd_controller::PdControllerConfig;

/// Exhaustive list of component types recognized
/// in .ibd v1 files. To add a new component: add 
/// a module here, define its Config (and initial 
/// if stateful), then add a variant below.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "component_type")]
pub enum ComponentKind {
    #[serde(rename = "pendulum")]
    Pendulum {
        config: PendulumConfig,
        initial: PendulumInitial,
    },
    #[serde(rename = "pd_controller")]
    PdController {
        config: PdControllerConfig,
    },
}
