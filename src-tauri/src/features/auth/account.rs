use crate::core::constants::{CHROME_USER_AGENT, YT_CLIENT_NAME, YT_CLIENT_VERSION, YT_ORIGIN};
use sha1::{Digest, Sha1};
use std::time::Duration;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoogleAccountInfo {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GuideResponse {
    #[serde(rename = "responseContext")]
    response_context: Option<GuideResponseContext>,
    #[serde(rename = "userProfile")]
    user_profile: Option<GuideUserProfile>,
}

#[derive(Debug, serde::Deserialize)]
struct GuideResponseContext {
    #[serde(rename = "mainAppWebResponseContext")]
    main_app_web_response_context: Option<GuideMainAppContext>,
}

#[derive(Debug, serde::Deserialize)]
struct GuideMainAppContext {
    #[serde(rename = "datasyncId")]
    datasync_id: Option<String>,
    #[serde(rename = "loggedOut")]
    logged_out: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
struct GuideUserProfile {
    #[serde(rename = "accountName")]
    account_name: Option<String>,
    email: Option<String>,
    #[serde(rename = "accountPhoto")]
    account_photo: Option<GuideAccountPhoto>,
}

#[derive(Debug, serde::Deserialize)]
struct GuideAccountPhoto {
    thumbnails: Option<Vec<GuideThumbnail>>,
}

#[derive(Debug, serde::Deserialize)]
struct GuideThumbnail {
    url: String,
}

pub async fn fetch_google_account(cookie_header: &str) -> Result<GoogleAccountInfo, String> {
    let sapisid = extract_cookie_value(cookie_header, "SAPISID")
        .or_else(|| extract_cookie_value(cookie_header, "__Secure-3PAPISID"))
        .or_else(|| extract_cookie_value(cookie_header, "__Secure-1PAPISID"))
        .ok_or_else(|| "No SAPISID cookie found in session".to_string())?;

    let client = reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let auth_header = format!("SAPISIDHASH {}", generate_sapisid_hash(&sapisid, YT_ORIGIN));

    let body = serde_json::json!({
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": YT_CLIENT_VERSION
            }
        }
    });

    let resp = client
        .post(format!("{YT_ORIGIN}/youtubei/v1/guide?prettyPrint=false"))
        .header("Cookie", cookie_header)
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Content-Type", "application/json")
        .header("Origin", YT_ORIGIN)
        .header("X-Origin", YT_ORIGIN)
        .header("Authorization", auth_header)
        .header("X-Youtube-Client-Name", YT_CLIENT_NAME)
        .header("X-Youtube-Client-Version", YT_CLIENT_VERSION)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("YouTube Guide API request failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("YouTube Guide API returned status {}", resp.status()));
    }

    let guide: GuideResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse Guide JSON: {e}"))?;

    let main_app = guide
        .response_context
        .and_then(|rc| rc.main_app_web_response_context);

    if let Some(ref ma) = main_app {
        if ma.logged_out == Some(true) {
            return Err("YouTube session is logged out".to_string());
        }
    }

    let user_profile = guide
        .user_profile
        .ok_or_else(|| "userProfile not found in YouTube response".to_string())?;

    let name = user_profile
        .account_name
        .unwrap_or_else(|| "Google User".to_string());

    let email = user_profile.email;

    let datasync_id = main_app
        .and_then(|ma| ma.datasync_id)
        .map(|s| s.trim_end_matches('|').to_string());

    let id = datasync_id
        .or_else(|| email.clone())
        .unwrap_or_else(|| format!("user_{}", &name));

    let avatar_url = user_profile
        .account_photo
        .and_then(|ap| ap.thumbnails)
        .and_then(|mut thumbs| thumbs.pop().map(|t| t.url));

    Ok(GoogleAccountInfo {
        id,
        name,
        email,
        avatar_url,
    })
}

fn generate_sapisid_hash(sapisid: &str, origin: &str) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut hasher = Sha1::new();
    hasher.update(format!("{now} {sapisid} {origin}").as_bytes());
    let hex_digest = format!("{:x}", hasher.finalize());
    format!("{now}_{hex_digest}")
}

fn extract_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    for part in cookie_header.split(';') {
        let part = part.trim();
        if let Some((k, v)) = part.split_once('=') {
            if k.trim() == name {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}
