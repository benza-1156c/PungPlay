use crate::utils::session;
use crate::YtdlpState;
use tauri::{AppHandle, Manager};
use tokio::process::Command;

pub async fn execute_ytdlp(app: &AppHandle, args: &[&str]) -> Result<String, String> {
    let _guard = session::SESSION_LOCK.lock().await;

    let ytdlp_state = app
        .try_state::<YtdlpState>()
        .ok_or_else(|| "yt-dlp ยังไม่พร้อมใช้งาน กรุณารอสักครู่".to_string())?;
    let ytdlp_path = &ytdlp_state.path;

    let cache_dir = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    tokio::fs::create_dir_all(&cache_dir)
        .await
        .map_err(|e| e.to_string())?;

    let temp_cookie_path = cache_dir.join("pungplay_cookies.txt");

    let cookie_header = session::get_cookie_header(app).await?;
    session::write_netscape_cookie_file(&cookie_header, &temp_cookie_path)?;

    let mut cmd = Command::new(ytdlp_path);
    cmd.args(["--cookies", temp_cookie_path.to_str().unwrap()]);
    cmd.args(args);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        if is_session_expired(&stderr) {
            session::handle_session_expired(app).await;
            return Err("GOOGLE_SESSION_EXPIRED: กรุณาล็อกอินใหม่".to_string());
        }
        return Err(format!("yt-dlp error: {}", stderr));
    }

    if !cookie_header.is_empty() {
        session::sync_from_netscape_file(app, &cookie_header, &temp_cookie_path).await;
    }

    Ok(stdout.to_string())
}

fn is_session_expired(stderr: &str) -> bool {
    stderr.contains("Sign in to confirm you’re not a bot")
        || stderr.contains("Sign in to confirm you're not a bot")
        || stderr.contains("Login required")
        || stderr.contains("HTTP Error 401: Unauthorized")
        || stderr.contains("Please sign in")
}
