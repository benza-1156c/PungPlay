use crate::features::auth::entity::{Column, Entity as Profile};
use crate::features::auth::repository;
use crate::YtdlpState;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::path::Path;
use tauri::{AppHandle, Manager};
use tokio::process::Command;
use tokio::sync::Mutex;

pub static COOKIE_MUTEX: Mutex<()> = Mutex::const_new(());

pub async fn execute_ytdlp(app: &AppHandle, args: &[&str]) -> Result<String, String> {
    let _guard = COOKIE_MUTEX.lock().await;

    let db = app.state::<DatabaseConnection>();
    let ytdlp_state = app.state::<YtdlpState>();
    let ytdlp_path = &ytdlp_state.path;

    let cache_dir = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    tokio::fs::create_dir_all(&cache_dir)
        .await
        .map_err(|e| e.to_string())?;

    let temp_cookie_path = cache_dir.join("pungplay_cookies.txt");

    let cookie_header = repository::get_active_cookie_header(&db).await?;
    write_cookies_to_netscape(&cookie_header, &temp_cookie_path)?;

    let mut cmd = Command::new(ytdlp_path);
    cmd.args(["--cookies", temp_cookie_path.to_str().unwrap()]);
    cmd.args(args);

    let output = cmd.output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        if let Ok(updated_cookies) = tokio::fs::read_to_string(&temp_cookie_path).await {
            let new_header = parse_netscape_to_header(&updated_cookies);
            let _ = update_db_cookie_header(&db, &new_header).await;
        }
    }

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn write_cookies_to_netscape(cookie_header: &str, path: &Path) -> Result<(), String> {
    let mut output = String::from("# Netscape HTTP Cookie File\n\n");

    for pair in cookie_header.split(';') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }

        if let Some((name, value)) = pair.split_once('=') {
            let name = name.trim();
            let value = value.trim();

            let domain = if name.starts_with("__Secure-")
                || name == "SID"
                || name == "HSID"
                || name == "SSID"
                || name == "APISID"
                || name == "SAPISID"
            {
                ".google.com"
            } else {
                ".youtube.com"
            };

            output.push_str(&format!(
                "{}\tTRUE\t/\tTRUE\t0\t{}\t{}\n",
                domain, name, value
            ));
        }
    }

    std::fs::write(path, output).map_err(|e| format!("Failed to write cookies: {e}"))?;
    Ok(())
}

fn parse_netscape_to_header(content: &str) -> String {
    let mut cookies = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 7 {
            let name = parts[5];
            let value = parts[6];
            cookies.push(format!("{}={}", name, value));
        }
    }

    cookies.join("; ")
}

async fn update_db_cookie_header(db: &DatabaseConnection, new_header: &str) -> Result<(), String> {
    let active = Profile::find()
        .filter(Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = active {
        let mut active_model: crate::features::auth::entity::ActiveModel = model.into();
        active_model.cookie_header = Set(new_header.to_string());
        active_model.update(db).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}
