use bevy::prelude::*;
use bevy_egui::EguiGlobalSettings;
use std::collections::HashMap;
use egui_tiles::{Tiles, Tree, Behavior, Linear, LinearDir};
use serde::{Serialize, Deserialize};
use bevy_egui::egui;

/// A top level "shell" plugin, 
/// that holds core data for the editor.
pub struct EditorShellPlugin;

impl Plugin for EditorShellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, editor_init_system);
    }
}

/// This is a trait that allows different plugins to implement their 
/// own rendering loop
pub trait EditorPanel: Send + Sync + 'static {
    fn name(&self) -> &str;
    /// Pass &mut World so the plugin planel can read/write 
    /// any game state they need.
    fn show(&mut self, ui: &mut egui::Ui, world: &World);
}

// The actual different types of panes that are available
// Rendered by plugins, but managed here so we can call 
// the right render function.
#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Default)]
enum EditorPanelType {
    #[default]
    Blank,
    Viewport(usize),
}
 
// Editor state that can be serialized. Saved 
// and used again in another session. 
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Serialize, Deserialize)]
struct PersistentState {}

// Editor state unique to this session. Cannot 
// be serialized nor used again.
#[derive(Debug, Clone)]
struct RuntimeState {
    panel_tree: Tree<EditorPanelType>,
}

#[derive(Resource, Debug, Clone)]
struct EditorState {
    persistent: PersistentState,
    runtime: RuntimeState,
}

fn editor_init_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {

    // Disable the automatic creation of a primary context
    // to set it up manually.
    egui_global_settings.auto_create_primary_context = false;

}
