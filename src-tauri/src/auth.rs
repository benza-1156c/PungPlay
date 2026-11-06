use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tokio::time::sleep;

pub const LOGIN_WINDOW_LABEL: &str = "youtube_login";
pub const LOGIN_SUCCESS_EVENT: &str = "login-success";
pub const LOGIN_CLOSED_EVENT: &str = "login-closed";

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
                        println!("[Auth] YouTube login completed successfully!");

                        let _ = app.emit(LOGIN_SUCCESS_EVENT, ());

                        let _ = login_win.close();
                        break;
                    }
                }
            } else {
                println!("[Auth] Login window was closed by the user.");
                let _ = app.emit(LOGIN_CLOSED_EVENT, ());
                break;
            }
        }
    });
}

#[tauri::command]
pub async fn login_youtube(app: AppHandle) -> Result<(), String> {
    let _window = get_or_create_login_window(&app)?;
    spawn_login_watcher(app);
    Ok(())
}
