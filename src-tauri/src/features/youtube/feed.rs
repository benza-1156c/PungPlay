use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::AppHandle;

use super::{client, parser};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct FeedVideo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub duration: Option<f64>,
    pub duration_text: Option<String>,
    pub channel: Option<String>,
    pub channel_avatar: Option<String>,
    pub view_count: Option<f64>,
    pub view_count_text: Option<String>,
    pub published_time: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct FeedResponse {
    pub videos: Vec<FeedVideo>,
    pub continuation: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_youtube_feed(
    app: AppHandle,
    continuation: Option<String>,
) -> Result<FeedResponse, String> {
    let payload = match &continuation {
        Some(token) => json!({ "continuation": token }),
        None => json!({ "browseId": "FEwhat_to_watch" }),
    };

    if let Ok(raw_json) = client::post_innertube(&app, "browse", payload).await {
        let _ = crate::utils::log::dump_json("raw_youtube_feed.json", &raw_json);
        let (videos, next_token) = parser::parse_feed_videos(&raw_json);
        if !videos.is_empty() {
            return Ok(FeedResponse {
                videos,
                continuation: next_token,
            });
        }
    }

    Ok(FeedResponse {
        videos: Vec::new(),
        continuation: None,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn search_youtube(
    app: AppHandle,
    query: String,
    continuation: Option<String>,
) -> Result<FeedResponse, String> {
    let payload = match &continuation {
        Some(token) => json!({ "continuation": token }),
        None => json!({ "query": query }),
    };

    let raw_json = client::post_innertube(&app, "search", payload).await?;
    let (videos, next_token) = parser::parse_feed_videos(&raw_json);
    Ok(FeedResponse {
        videos,
        continuation: next_token,
    })
}
