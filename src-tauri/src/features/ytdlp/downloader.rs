use crate::core::events::YtdlpEvent;
use futures_util::StreamExt;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tokio::io::AsyncWriteExt;

use super::detector::get_ytdlp_download_info;

pub async fn ensure_ytdlp_ready(app: &AppHandle) -> Result<PathBuf, String> {
    let bin_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {e}"))?
        .join("bin");

    tokio::fs::create_dir_all(&bin_dir)
        .await
        .map_err(|e| format!("Failed to create bin dir: {e}"))?;

    let (url, filename) = get_ytdlp_download_info();
    let ytdlp_path = bin_dir.join(filename);

    if ytdlp_path.exists() {
        YtdlpEvent::Ready.emit_event(app);
        return Ok(ytdlp_path);
    }

    println!("[Setup] Downloading yt-dlp from {url}");
    download_with_progress(url, &ytdlp_path, app).await?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&ytdlp_path)
            .await
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(&ytdlp_path, perms)
            .await
            .map_err(|e| e.to_string())?;
    }

    YtdlpEvent::Ready.emit_event(app);
    Ok(ytdlp_path)
}

async fn download_with_progress(url: &str, dst: &PathBuf, app: &AppHandle) -> Result<(), String> {
    let resp = reqwest::get(url).await.map_err(|e| {
        let err = format!("Request failed: {e}");
        YtdlpEvent::Error(err.clone()).emit_event(app);
        err
    })?;

    if !resp.status().is_success() {
        let err = format!("HTTP {}", resp.status());
        YtdlpEvent::Error(err.clone()).emit_event(app);
        return Err(err);
    }

    let total = resp.content_length().unwrap_or(0);
    let mut file = tokio::fs::File::create(dst).await.map_err(|e| {
        let err = format!("Failed to create file: {e}");
        YtdlpEvent::Error(err.clone()).emit_event(app);
        err
    })?;

    let mut stream = resp.bytes_stream();
    let mut downloaded = 0u64;
    let mut last_emit = std::time::Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| {
            let err = format!("Chunk error: {e}");
            YtdlpEvent::Error(err.clone()).emit_event(app);
            err
        })?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        if last_emit.elapsed().as_millis() >= 100 {
            YtdlpEvent::Progress {
                downloaded: downloaded as u32,
                total: total as u32,
            }
            .emit_event(app);
            last_emit = std::time::Instant::now();
        }
    }

    file.flush().await.map_err(|e| e.to_string())?;

    YtdlpEvent::Complete {
        total: downloaded as u32,
    }
    .emit_event(app);
    Ok(())
}
