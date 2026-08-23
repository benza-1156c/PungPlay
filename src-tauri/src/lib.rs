// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod models;
mod audio;

use tauri_specta::{collect_commands, Builder};
use audio::AudioState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            commands::search_youtube,
            commands::play_audio,
            commands::pause_audio,
            commands::resume_audio,
            commands::set_volume,
            commands::seek_audio,
            commands::get_audio_progress
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .manage(AudioState::new().expect("Failed to initialize rodio audio backend"))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
