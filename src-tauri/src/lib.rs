// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod auth;
mod models;

use models::events::AppEvent;
use tauri_specta::{collect_commands, collect_events, Builder};

#[cfg(debug_assertions)]
fn export_types(builder: &Builder<tauri::Wry>) {
    let mut export_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    if export_path.ends_with("src-tauri") {
        export_path.push("../src/lib/generated/bindings.ts");
    } else {
        export_path.push("src/lib/generated/bindings.ts");
    }
    if let Some(parent) = export_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    builder
        .export(specta_typescript::Typescript::default(), export_path)
        .expect("Failed to export typescript bindings");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![auth::login_youtube])
        .events(collect_events![AppEvent]);

    #[cfg(debug_assertions)]
    export_types(&builder);

    if std::env::args().any(|arg| arg == "--export-types") {
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init(), tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
