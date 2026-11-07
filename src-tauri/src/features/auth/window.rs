use crate::core::constants::{CHROME_USER_AGENT, LOGIN_WINDOW_LABEL, YOUTUBE_LOGIN_URL};
use crate::core::events::AuthEvent;
use crate::features::auth::account;
use std::time::Duration;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tokio::time::sleep;


pub fn is_youtube_home(url: &tauri::Url) -> bool {
    let host_str = url.host_str().unwrap_or("");
    let host_matches = host_str == "www.youtube.com" || host_str == "youtube.com";

    let url_str = url.as_str();
    let not_in_accounts = !url_str.contains("accounts.google.com");
    let not_in_signin = !url.path().contains("signin");

    host_matches && not_in_accounts && not_in_signin
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

    // Clean previous session directory so each login can authenticate a new account
    if login_profile_dir.exists() {
        let _ = std::fs::remove_dir_all(&login_profile_dir);
    }

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

                                match account::fetch_google_account(&yt_cookie_header).await {
                                    Ok(account_info) => {
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
                                    Err(e) => {
                                        eprintln!("[Auth] Failed to fetch account info: {e}");
                                        AuthEvent::Error(e).emit_event(&app);
                                    }
                                }
                            }
                            Err(e) => AuthEvent::Error(e.to_string()).emit_event(&app),
                        }
                        let _ = login_win.destroy();
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
