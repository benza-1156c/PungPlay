use crate::models::{ItemType, SearchResultItem};
use rusty_ytdl::search::{SearchOptions, SearchResult, YouTube};

#[tauri::command]
#[specta::specta]
pub async fn search_youtube(query: String) -> Result<Vec<SearchResultItem>, String> {
    if query.trim().is_empty() {
        return Err("กรุณาใส่คำค้นหา".to_string());
    }

    let youtube = YouTube::new().map_err(|e| format!("สร้าง YouTube Client ไม่สำเร็จ: {}", e))?;

    let options = SearchOptions {
        limit: 20,
        ..Default::default()
    };

    let results = youtube
        .search(query, Some(&options))
        .await
        .map_err(|e| format!("ค้นหาไม่สำเร็จ: {}", e))?;

    let items = results
        .into_iter()
        .map(|res| {
            let (item_type, id, title, thumbnails, url, artist, duration, views) = match res {
                SearchResult::Video(v) => (
                    ItemType::Video,
                    v.id,
                    v.title,
                    v.thumbnails,
                    v.url,
                    v.channel.name,
                    v.duration_raw,
                    v.views,
                ),
                SearchResult::Playlist(p) => (
                    ItemType::Playlist,
                    p.id,
                    p.name,
                    p.thumbnails,
                    p.url,
                    p.channel.name,
                    String::new(),
                    p.views,
                ),
                SearchResult::Channel(c) => (
                    ItemType::Channel,
                    c.id,
                    c.name,
                    c.icon,
                    c.url,
                    String::new(),
                    String::new(),
                    0,
                ),
            };

            let thumbnail = thumbnails
                .into_iter()
                .max_by_key(|t| t.width)
                .map_or_else(String::new, |t| t.url);

            SearchResultItem {
                item_type,
                id,
                title,
                thumbnail,
                url,
                artist,
                duration,
                views,
            }
        })
        .collect();

    Ok(items)
}
