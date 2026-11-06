use crate::features::auth::window::CHROME_USER_AGENT;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoogleAccountInfo {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

pub async fn fetch_google_account(
    yt_cookie_header: &str,
    google_cookie_header: &str,
) -> GoogleAccountInfo {
    let client = match reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .build()
    {
        Ok(c) => c,
        Err(_) => return generate_fallback_account(yt_cookie_header),
    };

    let mut info = fetch_from_youtube(&client, yt_cookie_header)
        .await
        .unwrap_or_else(|_| generate_fallback_account(yt_cookie_header));

    // Fetch real Google email using Google session cookies from SignOutOptions
    if let Some(email) = fetch_email_from_google(&client, google_cookie_header).await {
        info.email = Some(email);
    }

    info
}

async fn fetch_from_youtube(
    client: &reqwest::Client,
    cookie_header: &str,
) -> Result<GoogleAccountInfo, String> {

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

    let name = extract_between(&html, "\"USER_ACCOUNT_NAME\":\"", "\"")
        .or_else(|| extract_between(&html, "\"USER_NAME\":\"", "\""))
        .or_else(|| extract_between(&html, "\"accountName\":{\"simpleText\":\"", "\""));

    let id = extract_between(&html, "\"USER_SESSION_ID\":\"", "\"")
        .or_else(|| {
            extract_between(&html, "\"DATASYNC_ID\":\"", "\"")
                .map(|s| s.trim_end_matches('|').to_string())
        })
        .or_else(|| extract_between(&html, "\"channelId\":\"", "\""));

    let avatar_url = html
        .find("https://yt3.ggpht.com/")
        .or_else(|| html.find("https://yt3.googleusercontent.com/"))
        .or_else(|| html.find("https://lh3.googleusercontent.com/"))
        .and_then(|idx| {
            let slice = &html[idx..];
            let end_idx = slice.find('"').unwrap_or(slice.len());
            Some(slice[..end_idx].replace("&amp;", "&"))
        })
        .or_else(|| {
            extract_between(
                &html,
                "\"topbarMenuButtonRenderer\":{\"avatar\":{\"thumbnails\":[{\"url\":\"",
                "\"",
            )
        });

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

fn extract_between(haystack: &str, start_tag: &str, end_tag: &str) -> Option<String> {
    let start_idx = haystack.find(start_tag)? + start_tag.len();
    let end_idx = haystack[start_idx..].find(end_tag)?;
    let val = haystack[start_idx..start_idx + end_idx].trim();
    if val.is_empty() {
        None
    } else {
        Some(val.to_string())
    }
}

async fn fetch_email_from_google(
    client: &reqwest::Client,
    google_cookie_header: &str,
) -> Option<String> {
    if google_cookie_header.trim().is_empty() {
        return None;
    }

    let html = client
        .get("https://accounts.google.com/SignOutOptions")
        .header("Cookie", google_cookie_header)
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    // Method 1: class="account-email" ... >email</span>
    if let Some(pos) = html.find("class=\"account-email\"") {
        let rest = &html[pos..];
        if let Some(gt) = rest.find('>') {
            let after_gt = &rest[gt + 1..];
            if let Some(end) = after_gt.find("</span>") {
                let candidate = after_gt[..end].trim();
                if candidate.contains('@') {
                    return Some(candidate.to_string());
                }
            }
        }
    }

    // Method 2: name="Email" ... value="..."
    if let Some(pos) = html.find("name=\"Email\"") {
        let slice = &html[pos..std::cmp::min(html.len(), pos + 150)];
        if let Some(val_idx) = slice.find("value=\"") {
            let after_val = &slice[val_idx + 7..];
            if let Some(end) = after_val.find('"') {
                let email = after_val[..end].trim();
                if email.contains('@') {
                    return Some(email.to_string());
                }
            }
        }
    }

    // Method 3: id="account-...@..."
    if let Some(start) = html.find("id=\"account-") {
        let rest = &html[start + 12..];
        if let Some(end) = rest.find('"') {
            let candidate = rest[..end].trim();
            if candidate.contains('@') {
                return Some(candidate.to_string());
            }
        }
    }

    // Method 4: standard tag matches
    extract_between(&html, "data-email=\"", "\"")
        .or_else(|| extract_between(&html, "data-identifier=\"", "\""))
        .or_else(|| extract_between(&html, "\"email\":\"", "\""))
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
