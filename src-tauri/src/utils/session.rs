use crate::core::events::AuthEvent;
use crate::features::auth::{
    entity::{Column, Entity as Profile},
    repository,
    window::{get_or_create_login_window, spawn_login_watcher},
};
use chrono::Utc;
use reqwest::header::{HeaderMap, SET_COOKIE};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

pub static SESSION_LOCK: Mutex<()> = Mutex::const_new(());

pub use crate::core::constants::{YT_CLIENT_VERSION, YT_ORIGIN};

pub async fn get_cookie_header(app: &AppHandle) -> Result<String, String> {
    let db = app.state::<DatabaseConnection>();
    repository::get_active_cookie_header(&db).await
}

pub fn build_sapisidhash_header(cookie_header: &str) -> Option<String> {
    let sapisid = extract_cookie_value(cookie_header, "SAPISID")
        .or_else(|| extract_cookie_value(cookie_header, "__Secure-3PAPISID"))
        .or_else(|| extract_cookie_value(cookie_header, "__Secure-1PAPISID"))?;

    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let mut hasher = Sha1::new();
    hasher.update(format!("{now} {sapisid} {YT_ORIGIN}").as_bytes());
    let hex_digest = format!("{:x}", hasher.finalize());

    Some(format!("SAPISIDHASH {now}_{hex_digest}"))
}

pub fn extract_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    for part in cookie_header.split(';') {
        if let Some((k, v)) = part.trim().split_once('=') {
            if k.trim() == name {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

pub fn merge_cookie_strings(
    current_header: &str,
    updates: impl IntoIterator<Item = (String, String)>,
) -> Option<String> {
    let mut map = BTreeMap::new();
    for pair in current_header.split(';') {
        if let Some((k, v)) = pair.trim().split_once('=') {
            let k = k.trim();
            let v = v.trim();
            if !k.is_empty() {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }

    let mut changed = false;
    for (k, v) in updates {
        let key = k.trim();
        let val = v.trim();
        if key.is_empty() {
            continue;
        }
        if val.eq_ignore_ascii_case("deleted") || val.is_empty() {
            if map.remove(key).is_some() {
                changed = true;
            }
        } else if map.get(key).map(|old| old != val).unwrap_or(true) {
            map.insert(key.to_string(), val.to_string());
            changed = true;
        }
    }

    if !changed {
        return None;
    }

    Some(
        map.into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; "),
    )
}

pub async fn sync_from_http_headers(
    app: &AppHandle,
    current_cookie_header: &str,
    headers: &HeaderMap,
) {
    let mut updates = Vec::new();
    for val in headers.get_all(SET_COOKIE) {
        if let Ok(raw) = val.to_str() {
            if let Some(first_part) = raw.split(';').next() {
                if let Some((k, v)) = first_part.trim().split_once('=') {
                    updates.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
        }
    }

    if let Some(new_header) = merge_cookie_strings(current_cookie_header, updates) {
        let _ = save_updated_cookie_header(app, &new_header).await;
    }
}

pub async fn sync_from_netscape_file(app: &AppHandle, current_cookie_header: &str, path: &Path) {
    let Ok(content) = tokio::fs::read_to_string(path).await else {
        return;
    };

    let mut updates = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 7 {
            updates.push((parts[5].trim().to_string(), parts[6].trim().to_string()));
        }
    }

    if let Some(new_header) = merge_cookie_strings(current_cookie_header, updates) {
        let _ = save_updated_cookie_header(app, &new_header).await;
    }
}

pub fn write_netscape_cookie_file(cookie_header: &str, path: &Path) -> Result<(), String> {
    let mut output = String::from(
        "# Netscape HTTP Cookie File\n# https://curl.haxx.se/rfc/cookie_spec.html\n# This is a generated file! Do not edit.\n\n",
    );

    let expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + 31_536_000;

    for pair in cookie_header.split(';') {
        if let Some((name, value)) = pair.trim().split_once('=') {
            let name = name.trim();
            let value = value.trim();
            if !name.is_empty() {
                output.push_str(&format!(
                    ".youtube.com\tTRUE\t/\tTRUE\t{expires}\t{name}\t{value}\n"
                ));
            }
        }
    }

    std::fs::write(path, output).map_err(|e| format!("Failed to write cookies: {e}"))
}

pub async fn save_updated_cookie_header(app: &AppHandle, new_header: &str) -> Result<(), String> {
    let db = app.state::<DatabaseConnection>();
    Profile::update_many()
        .filter(Column::IsActive.eq(true))
        .col_expr(Column::CookieHeader, new_header.into())
        .col_expr(Column::UpdatedAt, Utc::now().into())
        .exec(&*db)
        .await
        .map_err(|e| format!("Failed to update cookie header: {e}"))?;
    Ok(())
}

pub async fn handle_session_expired(app: &AppHandle) {
    let db = app.state::<DatabaseConnection>();
    let _ = Profile::update_many()
        .filter(Column::IsActive.eq(true))
        .col_expr(Column::IsActive, false.into())
        .col_expr(Column::CookieHeader, "".into())
        .exec(&*db)
        .await;

    AuthEvent::Error("GOOGLE_SESSION_EXPIRED".to_string()).emit_event(app);

    if get_or_create_login_window(app).is_ok() {
        spawn_login_watcher(app.clone());
    }
}
