use crate::models::auth::AuthEvent;
use rusqlite::{Connection, OpenFlags};
use std::{fs, time::Duration};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tokio::time::sleep;

pub const LOGIN_WINDOW_LABEL: &str = "youtube_login";

pub const YOUTUBE_LOGIN_URL: &str =
    "https://accounts.google.com/ServiceLogin?service=youtube&continue=https://www.youtube.com/";
pub const CHROME_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

pub fn is_youtube_home(url: &tauri::Url) -> bool {
    let host_str = url.host_str().unwrap_or("");
    let host_matches = host_str == "www.youtube.com" || host_str == "youtube.com";

    let path = url.path();
    let path_is_home = path == "/" || path.is_empty();
    let not_in_accounts = !path.contains("accounts");

    host_matches && path_is_home && not_in_accounts
}

pub fn get_or_create_login_window(app: &AppHandle) -> Result<WebviewWindow, String> {
    if let Some(existing_window) = app.get_webview_window(LOGIN_WINDOW_LABEL) {
        let _ = existing_window.set_focus();
        return Ok(existing_window);
    }

    let parsed_url: tauri::Url = YOUTUBE_LOGIN_URL
        .parse()
        .map_err(|e| format!("Invalid login URL: {e}"))?;

    WebviewWindowBuilder::new(app, LOGIN_WINDOW_LABEL, WebviewUrl::External(parsed_url))
        .title("Login to YouTube")
        .inner_size(500.0, 500.0)
        .center()
        .user_agent(CHROME_USER_AGENT)
        .build()
        .map_err(|e| format!("Failed to create login window: {e}"))
}

pub fn spawn_login_watcher(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            sleep(Duration::from_millis(1500)).await;

            if let Some(login_win) = app.get_webview_window(LOGIN_WINDOW_LABEL) {
                if let Ok(url) = login_win.url() {
                    if is_youtube_home(&url) {
                        let _ = login_win.close();

                        sleep(Duration::from_millis(500)).await;

                        match extract_youtube_cookie(&app) {
                            Ok(cookie) => {
                                println!("{}", cookie);
                                AuthEvent::Success { cookie: cookie }.emit_event(&app);
                            }
                            Err(e) => AuthEvent::Error(e).emit_event(&app),
                        }

                        break;
                    }
                }
            } else {
                AuthEvent::Closed.emit_event(&app);
                break;
            }
        }
    });
}

#[tauri::command]
#[specta::specta]
pub async fn login_youtube(app: AppHandle) -> Result<(), String> {
    match get_or_create_login_window(&app) {
        Ok(_) => {
            spawn_login_watcher(app);
            Ok(())
        }
        Err(e) => {
            AuthEvent::Error(e.clone()).emit_event(&app);
            Err(e)
        }
    }
}

pub fn extract_youtube_cookie(app: &AppHandle) -> Result<String, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to get app data dir".to_string())?;

    let db_path = app_data_dir
        .join("EBWebView")
        .join("Default")
        .join("Network")
        .join("Cookies");

    if !db_path.exists() {
        return Err("ไม่พบไฟล์ฐานข้อมูล Cookies".to_string());
    }

    let temp_db_path = app_data_dir.join("Cookies_temp.sqlite");
    fs::copy(&db_path, &temp_db_path).map_err(|e| format!("Copy DB failed: {}", e))?;

    let conn = Connection::open_with_flags(&temp_db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| format!("Failed to open DB: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT value FROM cookies WHERE name = '__Secure-3PSID' AND host_key LIKE '%youtube.com%'"
    ).map_err(|e| format!("SQL Prepare failed: {}", e))?;

    let mut rows = stmt
        .query([])
        .map_err(|e| format!("SQL Query failed: {}", e))?;

    let result = if let Some(row) = rows.next().map_err(|e| format!("Row error: {}", e))? {
        let value: String = row.get(0).map_err(|e| format!("Get value failed: {}", e))?;
        Ok(value)
    } else {
        Err("__Secure-3PSID NotFound".to_string())
    };

    let _ = fs::remove_file(temp_db_path);

    return result;
}
