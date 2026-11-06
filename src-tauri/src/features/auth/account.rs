use crate::features::auth::window::CHROME_USER_AGENT;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoogleAccountInfo {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

pub async fn fetch_google_account(cookie_header: &str) -> GoogleAccountInfo {
    if let Ok(info) = fetch_from_youtube(cookie_header).await {
        return info;
    }
    generate_fallback_account(cookie_header)
}

async fn fetch_from_youtube(cookie_header: &str) -> Result<GoogleAccountInfo, String> {
    let client = reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

    let html = client
        .get("https://www.youtube.com")
        .header("Cookie", cookie_header)
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    let id = extract_pattern(&html, r#""channelId":"(UC[a-zA-Z0-9_-]+)""#)
        .or_else(|| extract_pattern(&html, r#""DATASYNC_ID":"([^"]+)""#))
        .or_else(|| extract_pattern(&html, r#""DELEGATED_SESSION_ID":"([^"]+)""#));

    let name = extract_pattern(&html, r#""USER_NAME":"([^"]+)""#)
        .or_else(|| extract_pattern(&html, r#""accountName":\{"simpleText":"([^"]+)""#))
        .or_else(|| extract_pattern(&html, r#""title":\{"simpleText":"([^"]+)""#));

    let avatar_url = extract_pattern(
        &html,
        r#""avatar":\{"thumbnails":\[\{"url":"(https://[^"]+)""#,
    )
    .or_else(|| extract_pattern(&html, r#""AVATAR_URL":"(https://[^"]+)""#));

    if let (Some(account_id), Some(account_name)) = (id, name) {
        return Ok(GoogleAccountInfo {
            id: account_id,
            name: account_name,
            email: None,
            avatar_url,
        });
    }

    Err("Could not extract account fields from YouTube page".to_string())
}

fn extract_pattern(haystack: &str, pattern: &str) -> Option<String> {
    if let Some(prefix_end) = pattern.find('(') {
        let prefix = &pattern[..prefix_end];
        let suffix = &pattern[pattern.find(')')? + 1..];

        if let Some(start_idx) = haystack.find(prefix) {
            let value_start = start_idx + prefix.len();
            if let Some(end_idx) = haystack[value_start..].find(suffix) {
                let value = &haystack[value_start..value_start + end_idx];
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn generate_fallback_account(cookie_header: &str) -> GoogleAccountInfo {
    let hash: u64 = cookie_header.bytes().fold(5381u64, |acc, b| {
        acc.wrapping_mul(33).wrapping_add(b as u64)
    });

    let short_id = format!("google_{:x}", hash & 0xFFFFFF);

    GoogleAccountInfo {
        id: short_id,
        name: "Google Account".to_string(),
        email: None,
        avatar_url: None,
    }
}
