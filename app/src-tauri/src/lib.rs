use ironbird_core::schema::{IronbirdProject, parse_ibd_project};
use tauri::{Manager, AppHandle};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// #[specta::specta]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

/// Wrapper type for anyhow errors so they can be 
/// sent across the wire for Tauri.
#[derive(Debug, serde::Serialize, specta::Type)]
pub struct CommandError(String);
impl From<anyhow::Error> for CommandError {
    fn from(e: anyhow::Error) -> Self {
        CommandError(e.to_string())
    }
}
type CommandResult<T> = Result<T, CommandError>;


#[tauri::command]
#[specta::specta]
fn load_demo_file_as_ibd(app: AppHandle, demo_name: String) -> CommandResult<IronbirdProject> {
    let resource_path = app.path()
        .resolve(format!("resources/demos/{}.ibd", demo_name), tauri::path::BaseDirectory::Resource)
        .map_err(anyhow::Error::from)?;

    let contents = std::fs::read_to_string(resource_path)
        .map_err(anyhow::Error::from)?;

    let loaded = parse_ibd_project(&contents)?;
    Ok(loaded)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            load_demo_file_as_ibd
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts"
        )
        .expect("Failed to export bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
