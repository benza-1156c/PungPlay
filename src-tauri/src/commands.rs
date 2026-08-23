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
        limit: 30,
        search_type: rusty_ytdl::search::SearchType::All,
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

            let mut thumbnail = thumbnails
                .into_iter()
                .max_by_key(|t| t.width)
                .map_or_else(String::new, |t| t.url);

            if thumbnail.starts_with("//") {
                thumbnail = format!("https:{}", thumbnail);
            }

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

use crate::audio::AudioState;
use std::{env, println};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn play_audio(video_id: String, state: State<'_, AudioState>) -> Result<(), String> {
    // ปิดระบบเล่นเพลงชั่วคราวตามที่ขอ
    // TODO: เปลี่ยนไปใช้ไลบรารีอื่นที่ใหม่กว่า (เช่น rustube หรือ yt-dlp)
    println!("Requested to play: {}", video_id);
    Err("ระบบเล่นเพลงกำลังปรับปรุง".to_string())
}

#[tauri::command]
#[specta::specta]
pub fn pause_audio(state: State<'_, AudioState>) {
    state.pause();
}

#[tauri::command]
#[specta::specta]
pub fn resume_audio(state: State<'_, AudioState>) {
    state.resume();
}

#[tauri::command]
#[specta::specta]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) {
    state.set_volume(volume);
}

#[tauri::command]
#[specta::specta]
pub fn seek_audio(position: f32, state: State<'_, AudioState>) {
    state.seek(position);
}

#[tauri::command]
#[specta::specta]
pub fn get_audio_progress(state: State<'_, AudioState>) -> f32 {
    state.get_position()
}
