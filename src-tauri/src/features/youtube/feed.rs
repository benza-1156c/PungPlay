use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::AppHandle;

use super::{client, models::ExtractedInfo, parser};
use crate::utils::ytdlp::execute_ytdlp;

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

#[tauri::command]
#[specta::specta]
pub async fn fetch_youtube_feed(app: AppHandle) -> Result<Vec<FeedVideo>, String> {
    if let Ok(raw_json) =
        client::post_innertube(&app, "browse", json!({ "browseId": "FEwhat_to_watch" })).await
    {
        let _ = crate::utils::log::dump_json("raw_youtube_feed.json", &raw_json);
        let (videos, _) = parser::parse_feed_videos(&raw_json);
        if !videos.is_empty() {
            return Ok(videos);
        }
    }
    fetch_feed_via_ytdlp(&app).await
}

#[tauri::command]
#[specta::specta]
pub async fn search_youtube(app: AppHandle, query: String) -> Result<Vec<FeedVideo>, String> {
    let raw_json = client::post_innertube(&app, "search", json!({ "query": query })).await?;
    let (videos, _) = parser::parse_feed_videos(&raw_json);
    Ok(videos)
}

async fn fetch_feed_via_ytdlp(app: &AppHandle) -> Result<Vec<FeedVideo>, String> {
    let stdout = execute_ytdlp(
        app,
        &[
            "--flat-playlist",
            "--dump-json",
            "--no-warnings",
            "--playlist-end",
            "30",
            ":ytrec",
        ],
    )
    .await?;

    let mut videos = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(info) = serde_json::from_str::<ExtractedInfo>(line) {
            if info.id.starts_with("RD") || info.extractor_key.as_deref() == Some("YoutubeTab") {
                continue;
            }
            let thumbnail_url = info.thumbnail.or_else(|| {
                info.thumbnails
                    .and_then(|t| t.last().map(|x| x.url.clone()))
            });

            videos.push(FeedVideo {
                id: info.id.clone(),
                title: info.title.unwrap_or_else(|| "Unknown Title".to_string()),
                url: info
                    .webpage_url
                    .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={}", info.id)),
                duration: info.duration,
                duration_text: info.duration_string,
                channel: info.channel.or(info.uploader),
                channel_avatar: None,
                view_count: info.view_count,
                view_count_text: None,
                published_time: None,
                thumbnail: thumbnail_url,
            });
        }
    }

    Ok(videos)
}
