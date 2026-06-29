use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use ironbird_editor::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(ironbird_editor::EditorUiPlugin)
        .run();
}
