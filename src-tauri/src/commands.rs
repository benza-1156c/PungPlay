use crate::models::{ItemType, SearchResultItem};
use rusty_ytdl::search::{SearchOptions, SearchResult, YouTube};

#[specta::specta]
#[tauri::command]
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
        .map(|res| match res {
            SearchResult::Video(v) => {
                let thumbnail_url = v
                    .thumbnails
                    .into_iter()
                    .max_by_key(|t| t.width)
                    .map_or("".to_string(), |t| t.url);

                SearchResultItem {
                    item_type: ItemType::Video,
                    id: v.id,
                    title: v.title,
                    thumbnail: thumbnail_url,
                    url: v.url,
                    artist: v.channel.name,
                    duration: v.duration_raw,
                    views: v.views,
                }
            }
            SearchResult::Playlist(p) => {
                let thumbnail_url = p
                    .thumbnails
                    .into_iter()
                    .max_by_key(|t| t.width)
                    .map_or("".to_string(), |t| t.url);

                SearchResultItem {
                    item_type: ItemType::Playlist,
                    id: p.id,
                    title: p.name,
                    thumbnail: thumbnail_url,
                    url: p.url,
                    artist: p.channel.name,
                    duration: "".to_string(),
                    views: p.views,
                }
            }
            SearchResult::Channel(c) => {
                let thumbnail_url = c
                    .icon
                    .into_iter()
                    .max_by_key(|t| t.width)
                    .map_or("".to_string(), |t| t.url);

                SearchResultItem {
                    item_type: ItemType::Channel,
                    id: c.id,
                    title: c.name,
                    thumbnail: thumbnail_url,
                    url: c.url,
                    artist: "".to_string(),
                    duration: "".to_string(),
                    views: 0,
                }
            }
        })
        .collect();

    Ok(items)
}
