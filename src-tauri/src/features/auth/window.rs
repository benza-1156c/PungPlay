use crate::core::events::AuthEvent;
use crate::features::auth::account;
use std::time::Duration;
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

    let login_profile_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Failed to get local data dir".to_string())?
        .join("LoginProfile");

    WebviewWindowBuilder::new(app, LOGIN_WINDOW_LABEL, WebviewUrl::External(parsed_url))
        .title("Login to YouTube")
        .inner_size(500.0, 600.0)
        .center()
        .user_agent(CHROME_USER_AGENT)
        .data_directory(login_profile_dir)
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
                        match login_win.cookies() {
                            Ok(cookies) => {
                                let has_auth = cookies.iter().any(|c| {
                                    let name = c.name();
                                    (name == "LOGIN_INFO" || name == "SAPISID" || name == "SID")
                                        && c.domain().unwrap_or("").contains("youtube.com")
                                });

                                if !has_auth {
                                    continue;
                                }

                                let yt_cookie_header =
                                    crate::features::auth::repository::build_cookie_header(
                                        &cookies,
                                    );
                                let google_cookie_header =
                                    crate::features::auth::repository::build_google_cookie_header(
                                        &cookies,
                                    );

                                let account_info =
                                    account::fetch_google_account(&yt_cookie_header, &google_cookie_header).await;
                                println!(
                                    "[Auth] Logged in as: {} ({}, email: {:?})",
                                    account_info.name, account_info.id, account_info.email
                                );

                                let db = app.state::<sea_orm::DatabaseConnection>();
                                match crate::features::auth::repository::save_profile_cookies(
                                    &db,
                                    &account_info.id,
                                    &account_info.name,
                                    account_info.email,
                                    account_info.avatar_url,
                                    &cookies,
                                )
                                .await
                                {
                                    Ok(saved_header) => {
                                        AuthEvent::Success {
                                            cookie: saved_header,
                                        }
                                        .emit_event(&app);
                                    }
                                    Err(e) => {
                                        eprintln!("[Auth] Failed to save profile: {e}");
                                        AuthEvent::Error(e).emit_event(&app);
                                    }
                                }
                            }
                            Err(e) => AuthEvent::Error(e.to_string()).emit_event(&app),
                        }
                        let _ = login_win.close();
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
