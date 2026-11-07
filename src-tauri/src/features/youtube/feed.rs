use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::models::ExtractedInfo;
use crate::utils::ytdlp::execute_ytdlp;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct FeedVideo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub duration: Option<f64>,
    pub channel: Option<String>,
    pub view_count: Option<f64>,
    pub thumbnail: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_youtube_feed(app: AppHandle) -> Result<Vec<FeedVideo>, String> {
    let stdout = execute_ytdlp(
        &app,
        &[
            "--flat-playlist",
            "--dump-json",
            "--no-warnings",
            "--buffer-size",
            "512K",
            "--concurrent-fragments",
            "1",
            "https://www.youtube.com/feed/subscriptions",
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
            let thumbnail_url = if let Some(t) = info.thumbnail {
                Some(t)
            } else if let Some(thumbs) = info.thumbnails {
                thumbs.last().map(|t| t.url.clone())
            } else {
                None
            };

            let video = FeedVideo {
                id: info.id.clone(),
                title: info.title.unwrap_or_else(|| "Unknown Title".to_string()),
                url: info
                    .webpage_url
                    .unwrap_or_else(|| format!("https://youtube.com/watch?v={}", info.id.clone())),
                duration: info.duration,
                channel: info.channel.or(info.uploader),
                view_count: info.view_count,
                thumbnail: thumbnail_url,
            };

            videos.push(video);
        }
    }

    Ok(videos)
}
