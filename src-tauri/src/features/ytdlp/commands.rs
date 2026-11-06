use super::detector::get_ytdlp_download_info;
use super::downloader::ensure_ytdlp_ready;
use tauri::{AppHandle, Manager};

#[tauri::command]
#[specta::specta]
pub async fn check_ytdlp_ready(app: AppHandle) -> Result<bool, String> {
    let bin_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("bin");
    let (_, filename) = get_ytdlp_download_info();
    Ok(bin_dir.join(filename).exists())
}

#[tauri::command]
#[specta::specta]
pub async fn trigger_ytdlp_download(app: AppHandle) -> Result<(), String> {
    ensure_ytdlp_ready(&app).await?;
    Ok(())
}
