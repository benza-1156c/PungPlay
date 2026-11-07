// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[macro_use]
mod commands;
pub mod core;
pub mod features;
pub mod utils;

use core::events::AppEvent;
use tauri::Manager;
use tauri_specta::{collect_events, Builder};

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
        .commands(app_commands!())
        .events(collect_events![AppEvent]);

    #[cfg(debug_assertions)]
    export_types(&builder);

    if std::env::args().any(|arg| arg == "--export-types") {
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let db = core::db::init_db(&handle)
                    .await
                    .expect("Failed to initialize SQLite database via SeaORM");
                handle.manage(db);
            });

            // Init yt-dlp
            let ytdlp_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match features::ytdlp::downloader::ensure_ytdlp_ready(&ytdlp_handle).await {
                    Ok(path) => {
                        ytdlp_handle.manage(YtdlpState { path });
                    }
                    Err(e) => {
                        eprintln!("[Setup] Failed to ensure yt-dlp: {e}");
                        core::events::YtdlpEvent::Error(e).emit_event(&ytdlp_handle);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct YtdlpState {
    pub path: std::path::PathBuf,
}
