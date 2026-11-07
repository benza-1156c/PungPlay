use crate::core::constants::{CHROME_USER_AGENT, YT_CLIENT_NAME, YT_CLIENT_VERSION, YT_ORIGIN};
use crate::utils::session;
use serde_json::{json, Value};
use std::time::Duration;
use tauri::AppHandle;

pub async fn post_innertube(
    app: &AppHandle,
    endpoint: &str,
    mut payload: Value,
) -> Result<Value, String> {
    let _guard = session::SESSION_LOCK.lock().await;

    let cookie_header = session::get_cookie_header(app).await?;

    payload["context"] = json!({
        "client": {
            "hl": "en",
            "gl": "US",
            "clientName": "WEB",
            "clientVersion": YT_CLIENT_VERSION,
            "userAgent": CHROME_USER_AGENT
        }
    });

    let client = reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let url = format!("{YT_ORIGIN}/youtubei/v1/{endpoint}?prettyPrint=false");
    let mut req = client
        .post(&url)
        .header("Origin", YT_ORIGIN)
        .header("X-Origin", YT_ORIGIN)
        .header("Referer", format!("{YT_ORIGIN}/"))
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("X-YouTube-Client-Name", YT_CLIENT_NAME)
        .header("X-YouTube-Client-Version", YT_CLIENT_VERSION);

    if !cookie_header.is_empty() {
        req = req.header("Cookie", &cookie_header);
        if let Some(auth) = session::build_sapisidhash_header(&cookie_header) {
            req = req.header("Authorization", auth);
        }
    }

    let res = req
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("YouTube API request failed: {e}"))?;

    if res.status() == reqwest::StatusCode::UNAUTHORIZED {
        session::handle_session_expired(app).await;
        return Err("GOOGLE_SESSION_EXPIRED".to_string());
    }

    if !cookie_header.is_empty() {
        session::sync_from_http_headers(app, &cookie_header, res.headers()).await;
    }

    if !res.status().is_success() {
        return Err(format!("YouTube API returned HTTP {}", res.status()));
    }

    let body: Value = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse YouTube JSON: {e}"))?;

    let is_logged_out = body
        .pointer("/responseContext/mainAppWebResponseContext/loggedOut")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !cookie_header.is_empty() && is_logged_out {
        session::handle_session_expired(app).await;
        return Err("GOOGLE_SESSION_EXPIRED".to_string());
    }

    Ok(body)
}
