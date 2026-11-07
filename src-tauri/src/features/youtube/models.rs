use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 1. Custom Types & Constants
// ============================================================================

pub type ExtractedType = String;
pub const EXTRACTED_TYPE_ANY: &str = "any";
pub const EXTRACTED_TYPE_SINGLE: &str = "single";
pub const EXTRACTED_TYPE_VIDEO: &str = "video";
pub const EXTRACTED_TYPE_PLAYLIST: &str = "playlist";
pub const EXTRACTED_TYPE_MULTI_VIDEO: &str = "multi_video";
pub const EXTRACTED_TYPE_URL: &str = "url";
pub const EXTRACTED_TYPE_URL_TRANSPARENT: &str = "url_transparent";

pub type ExtractedLiveStatus = String;
pub const EXTRACTED_LIVE_STATUS_IS_LIVE: &str = "is_live";
pub const EXTRACTED_LIVE_STATUS_IS_UPCOMING: &str = "is_upcoming";
pub const EXTRACTED_LIVE_STATUS_WAS_LIVE: &str = "was_live";
pub const EXTRACTED_LIVE_STATUS_NOT_LIVE: &str = "not_live";
pub const EXTRACTED_LIVE_STATUS_POST_LIVE: &str = "post_live";

pub type ExtractedAvailability = String;
pub const EXTRACTED_AVAILABILITY_PRIVATE: &str = "private";
pub const EXTRACTED_AVAILABILITY_PREMIUM_ONLY: &str = "premium_only";
pub const EXTRACTED_AVAILABILITY_SUBSCRIBER_ONLY: &str = "subscriber_only";
pub const EXTRACTED_AVAILABILITY_NEEDS_AUTH: &str = "needs_auth";
pub const EXTRACTED_AVAILABILITY_UNLISTED: &str = "unlisted";
pub const EXTRACTED_AVAILABILITY_PUBLIC: &str = "public";

// ============================================================================
// 2. Result Structs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    #[serde(rename = "executable")]
    pub executable: String,
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "exit_code")]
    pub exit_code: i32,
    #[serde(rename = "stdout")]
    pub stdout: String,
    #[serde(rename = "stderr")]
    pub stderr: String,
    #[serde(rename = "output_logs")]
    pub output_logs: Option<Vec<ResultLog>>,
}

// Placeholder สำหรับ ResultLog เนื่องจากไม่ได้ประกาศในไฟล์นี้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultLog {
    pub pipe: String,
    pub json: Option<serde_json::Value>,
}

impl CommandResult {
    pub fn as_string(
        &self,
        stdout: bool,
        stderr: bool,
        _timestamps: bool,
        _mask_json: bool,
        exit_code: bool,
    ) -> String {
        let mut out = Vec::new();
        if let Some(logs) = &self.output_logs {
            for l in logs {
                if l.pipe == "stdout" && !stdout {
                    continue;
                }
                if l.pipe == "stderr" && !stderr {
                    continue;
                }
                out.push(format!("{:?}", l));
            }
        }
        if exit_code {
            out.push(format!("exit code: {}", self.exit_code));
        }
        out.join("\n")
    }

    pub fn get_extracted_info(&self) -> Result<Vec<ExtractedInfo>, String> {
        let mut infos = Vec::new();
        if let Some(logs) = &self.output_logs {
            for log in logs {
                if let Some(json_val) = &log.json {
                    let json_str = json_val.to_string();
                    let e = parse_extracted_info(&json_str)?;
                    if e.type_.is_empty() {
                        continue;
                    }
                    infos.push(e);
                }
            }
        }
        Ok(infos)
    }
}

impl std::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string(true, true, true, true, true))
    }
}

// ============================================================================
// 3. Core Parsing & Reflection Logic
// ============================================================================

pub fn parse_extracted_info(msg: &str) -> Result<ExtractedInfo, String> {
    let mut val: serde_json::Value = serde_json::from_str(msg).map_err(|e| e.to_string())?;
    clean_json_value(&mut val);
    let info: ExtractedInfo = serde_json::from_value(val).map_err(|e| e.to_string())?;
    Ok(info)
}

pub fn clean_json_value(v: &mut serde_json::Value) {
    match v {
        serde_json::Value::Object(map) => {
            for (k, val) in map.iter_mut() {
                if k == "title" {
                    if val.is_string() && (val.as_str() == Some("none") || val.as_str() == Some(""))
                    {
                        *val = serde_json::Value::String("".to_string());
                        continue;
                    }
                }

                if val.is_string() && val.as_str() == Some("none") {
                    *val = serde_json::Value::Null;
                } else {
                    clean_json_value(val);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for val in arr.iter_mut() {
                clean_json_value(val);
            }
        }
        serde_json::Value::String(s) => {
            if s == "none" {
                *v = serde_json::Value::Null;
            }
        }
        _ => {}
    }
}

// ============================================================================
// 4. ExtractedInfo (Main Metadata)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedInfo {
    // Embedded ExtractedFormat
    #[serde(flatten)]
    pub format_info: Option<ExtractedFormat>,

    #[serde(rename = "_type")]
    pub type_: ExtractedType,

    #[serde(rename = "_version")]
    pub version: Option<ExtractedVersion>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "formats")]
    pub formats: Option<Vec<ExtractedFormat>>,

    #[serde(rename = "requested_formats")]
    pub requested_formats: Option<Vec<ExtractedFormat>>,

    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "filename")]
    pub filename: Option<String>,

    #[serde(rename = "_filename")]
    pub alt_filename: Option<String>,

    #[serde(rename = "ext")]
    pub extension: String,

    #[serde(rename = "format")]
    pub format: String,

    #[serde(rename = "player_url")]
    pub player_url: Option<String>,

    #[serde(rename = "direct")]
    pub direct: Option<bool>,

    #[serde(rename = "alt_title")]
    pub alt_title: Option<String>,

    #[serde(rename = "display_id")]
    pub display_id: Option<String>,

    #[serde(rename = "thumbnails")]
    pub thumbnails: Option<Vec<ExtractedThumbnail>>,

    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "uploader")]
    pub uploader: Option<String>,

    #[serde(rename = "license")]
    pub license: Option<String>,

    #[serde(rename = "creator")]
    pub creator: Option<String>,

    #[serde(rename = "creators")]
    pub creators: Option<Vec<String>>,

    #[serde(rename = "timestamp")]
    pub timestamp: Option<f64>,

    #[serde(rename = "upload_date")]
    pub upload_date: Option<String>,

    #[serde(rename = "release_timestamp")]
    pub release_timestamp: Option<f64>,

    #[serde(rename = "release_date")]
    pub release_date: Option<String>,

    #[serde(rename = "release_year")]
    pub release_year: Option<i32>,

    #[serde(rename = "modified_timestamp")]
    pub modified_timestamp: Option<f64>,

    #[serde(rename = "modified_date")]
    pub modified_date: Option<String>,

    #[serde(rename = "uploader_id")]
    pub uploader_id: Option<String>,

    #[serde(rename = "uploader_url")]
    pub uploader_url: Option<String>,

    #[serde(rename = "channel")]
    pub channel: Option<String>,

    #[serde(rename = "channel_id")]
    pub channel_id: Option<String>,

    #[serde(rename = "channel_url")]
    pub channel_url: Option<String>,

    #[serde(rename = "channel_follower_count")]
    pub channel_follower_count: Option<f64>,

    #[serde(rename = "channel_is_verified")]
    pub channel_is_verified: Option<bool>,

    #[serde(rename = "location")]
    pub location: Option<String>,

    #[serde(rename = "subtitles")]
    pub subtitles: Option<HashMap<String, Vec<ExtractedSubtitle>>>,

    #[serde(rename = "requested_subtitles")]
    pub requested_subtitles: Option<HashMap<String, ExtractedSubtitle>>,

    #[serde(rename = "automatic_captions")]
    pub automatic_captions: Option<HashMap<String, Vec<ExtractedSubtitle>>>,

    #[serde(rename = "duration")]
    pub duration: Option<f64>,

    #[serde(rename = "view_count")]
    pub view_count: Option<f64>,

    #[serde(rename = "concurrent_view_count")]
    pub concurrent_view_count: Option<f64>,

    #[serde(rename = "save_count")]
    pub save_count: Option<f64>,

    #[serde(rename = "like_count")]
    pub like_count: Option<f64>,

    #[serde(rename = "dislike_count")]
    pub dislike_count: Option<f64>,

    #[serde(rename = "repost_count")]
    pub repost_count: Option<f64>,

    #[serde(rename = "average_rating")]
    pub average_rating: Option<f64>,

    #[serde(rename = "comment_count")]
    pub comment_count: Option<f64>,

    #[serde(rename = "comments")]
    pub comments: Option<Vec<ExtractedVideoComment>>,

    #[serde(rename = "age_limit")]
    pub age_limit: Option<f64>,

    #[serde(rename = "webpage_url")]
    pub webpage_url: Option<String>,

    #[serde(rename = "categories")]
    pub categories: Option<Vec<String>>,

    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "cast")]
    pub cast: Option<Vec<String>>,

    #[serde(rename = "is_live")]
    pub is_live: Option<bool>,

    #[serde(rename = "was_live")]
    pub was_live: Option<bool>,

    #[serde(rename = "live_status")]
    pub live_status: Option<ExtractedLiveStatus>,

    #[serde(rename = "start_time")]
    pub start_time: Option<f64>,

    #[serde(rename = "end_time")]
    pub end_time: Option<f64>,

    #[serde(rename = "chapters")]
    pub chapters: Option<Vec<ExtractedChapterData>>,

    #[serde(rename = "heatmap")]
    pub heatmap: Option<Vec<ExtractedHeatmapData>>,

    #[serde(rename = "playable_in_embed")]
    pub playable_in_embed: Option<serde_json::Value>,

    #[serde(rename = "availability")]
    pub availability: Option<ExtractedAvailability>,

    #[serde(rename = "media_type")]
    pub media_type: Option<String>,

    #[serde(rename = "chapter")]
    pub chapter: Option<String>,

    #[serde(rename = "chapter_number")]
    pub chapter_number: Option<f64>,

    #[serde(rename = "chapter_id")]
    pub chapter_id: Option<String>,

    #[serde(rename = "playlist")]
    pub playlist: Option<String>,

    #[serde(rename = "playlist_index")]
    pub playlist_index: Option<i32>,

    #[serde(rename = "playlist_id")]
    pub playlist_id: Option<String>,

    #[serde(rename = "playlist_title")]
    pub playlist_title: Option<String>,

    #[serde(rename = "playlist_uploader")]
    pub playlist_uploader: Option<String>,

    #[serde(rename = "playlist_uploader_id")]
    pub playlist_uploader_id: Option<String>,

    #[serde(rename = "playlist_count")]
    pub playlist_count: Option<i32>,

    #[serde(rename = "series")]
    pub series: Option<String>,

    #[serde(rename = "series_id")]
    pub series_id: Option<String>,

    #[serde(rename = "season")]
    pub season: Option<String>,

    #[serde(rename = "season_number")]
    pub season_number: Option<f64>,

    #[serde(rename = "season_id")]
    pub season_id: Option<String>,

    #[serde(rename = "episode")]
    pub episode: Option<String>,

    #[serde(rename = "episode_number")]
    pub episode_number: Option<f64>,

    #[serde(rename = "episode_id")]
    pub episode_id: Option<String>,

    #[serde(rename = "track")]
    pub track: Option<String>,

    #[serde(rename = "track_number")]
    pub track_number: Option<f64>,

    #[serde(rename = "track_id")]
    pub track_id: Option<String>,

    #[serde(rename = "artists")]
    pub artists: Option<Vec<String>>,

    #[serde(rename = "artist")]
    pub artist: Option<String>,

    #[serde(rename = "composers")]
    pub composers: Option<Vec<String>>,

    #[serde(rename = "genres")]
    pub genres: Option<Vec<String>>,

    #[serde(rename = "genre")]
    pub genre: Option<String>,

    #[serde(rename = "album")]
    pub album: Option<String>,

    #[serde(rename = "album_type")]
    pub album_type: Option<String>,

    #[serde(rename = "album_artists")]
    pub album_artists: Option<Vec<String>>,

    #[serde(rename = "album_artist")]
    pub album_artist: Option<String>,

    #[serde(rename = "disc_number")]
    pub disc_number: Option<f64>,

    #[serde(rename = "composer")]
    pub composer: Option<String>,

    #[serde(rename = "section_start")]
    pub section_start: Option<f64>,

    #[serde(rename = "section_end")]
    pub section_end: Option<f64>,

    #[serde(rename = "rows")]
    pub rows: Option<i32>,

    #[serde(rename = "columns")]
    pub columns: Option<i32>,

    #[serde(rename = "extractor")]
    pub extractor: Option<String>,

    #[serde(rename = "extractor_key")]
    pub extractor_key: Option<String>,

    #[serde(rename = "webpage_url_basename")]
    pub webpage_url_basename: Option<String>,

    #[serde(rename = "webpage_url_domain")]
    pub webpage_url_domain: Option<String>,

    #[serde(rename = "autonumber")]
    pub autonumber: i32,

    #[serde(rename = "epoch")]
    pub epoch: Option<f64>,

    #[serde(rename = "entries")]
    pub entries: Option<Vec<Box<ExtractedInfo>>>,
}

// ============================================================================
// 5. ExtractedFormat
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFormat {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "request_data")]
    pub request_data: Option<String>,

    #[serde(rename = "manifest_stream_number")]
    pub manifest_stream_number: Option<i32>,

    #[serde(rename = "manifest_url")]
    pub manifest_url: Option<String>,

    #[serde(rename = "ext")]
    pub extension: Option<String>,

    #[serde(rename = "format")]
    pub format: Option<String>,

    #[serde(rename = "format_id")]
    pub format_id: Option<String>,

    #[serde(rename = "format_note")]
    pub format_note: Option<String>,

    #[serde(rename = "width")]
    pub width: Option<f64>,

    #[serde(rename = "height")]
    pub height: Option<f64>,

    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Option<f64>,

    #[serde(rename = "resolution")]
    pub resolution: Option<String>,

    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Option<String>,

    #[serde(rename = "tbr")]
    pub tbr: Option<f64>,

    #[serde(rename = "abr")]
    pub abr: Option<f64>,

    #[serde(rename = "acodec")]
    pub acodec: Option<String>,

    #[serde(rename = "asr")]
    pub asr: Option<f64>,

    #[serde(rename = "audio_channels")]
    pub audio_channels: Option<f64>,

    #[serde(rename = "vbr")]
    pub vbr: Option<f64>,

    #[serde(rename = "fps")]
    pub fps: Option<f64>,

    #[serde(rename = "vcodec")]
    pub vcodec: Option<String>,

    #[serde(rename = "container")]
    pub container: Option<String>,

    #[serde(rename = "filesize")]
    pub filesize: Option<i64>,

    #[serde(rename = "filesize_approx")]
    pub filesize_approx: Option<i64>,

    #[serde(rename = "player_url")]
    pub player_url: Option<String>,

    #[serde(rename = "protocol")]
    pub protocol: Option<String>,

    #[serde(rename = "fragment_base_url")]
    pub fragment_base_url: Option<String>,

    #[serde(rename = "fragments")]
    pub fragments: Option<Vec<ExtractedFragment>>,

    #[serde(rename = "hls_media_playlist_data")]
    pub hls_media_playlist_data: Option<String>,

    #[serde(rename = "is_from_start")]
    pub is_from_start: Option<bool>,

    #[serde(rename = "preference")]
    pub preference: Option<i32>,

    #[serde(rename = "source_preference")]
    pub source_preference: Option<i32>,

    #[serde(rename = "language")]
    pub language: Option<String>,

    #[serde(rename = "language_preference")]
    pub language_preference: Option<i32>,

    #[serde(rename = "quality")]
    pub quality: Option<f64>,

    #[serde(rename = "http_headers")]
    pub http_headers: Option<HashMap<String, String>>,

    #[serde(rename = "stretched_ratio")]
    pub stretched_ratio: Option<f64>,

    #[serde(rename = "no_resume")]
    pub no_resume: Option<bool>,

    #[serde(rename = "has_drm")]
    pub has_drm: Option<serde_json::Value>,

    #[serde(rename = "extra_param_to_segment_url")]
    pub extra_param_to_segment_url: Option<String>,

    #[serde(rename = "extra_param_to_key_url")]
    pub extra_param_to_key_url: Option<String>,

    #[serde(rename = "hls_aes")]
    pub hls_aes: Option<ExtractedHLSAES>,

    #[serde(rename = "impersonate")]
    pub impersonate: Option<serde_json::Value>,

    #[serde(rename = "available_at")]
    pub available_at: Option<f64>,

    #[serde(rename = "http_chunk_size")]
    pub http_chunk_size: Option<i32>,

    #[serde(rename = "ffmpeg_args")]
    pub ffmpeg_args: Option<Vec<String>>,

    #[serde(rename = "ffmpeg_args_out")]
    pub ffmpeg_args_out: Option<Vec<String>>,

    #[serde(rename = "ws_url")]
    pub ws_url: Option<String>,

    #[serde(rename = "max_quality")]
    pub max_quality: Option<String>,

    #[serde(rename = "is_dash_periods")]
    pub is_dash_periods: Option<bool>,

    #[serde(rename = "rows")]
    pub rows: Option<i32>,

    #[serde(rename = "columns")]
    pub columns: Option<i32>,

    #[serde(rename = "page_url")]
    pub page_url: Option<String>,

    #[serde(rename = "app")]
    pub app: Option<String>,

    #[serde(rename = "play_path")]
    pub play_path: Option<String>,

    #[serde(rename = "tc_url")]
    pub tc_url: Option<String>,

    #[serde(rename = "flash_version")]
    pub flash_version: Option<String>,

    #[serde(rename = "rtmp_live")]
    pub rtmp_live: Option<bool>,

    #[serde(rename = "rtmp_conn")]
    pub rtmp_conn: Option<String>,

    #[serde(rename = "rtmp_protocol")]
    pub rtmp_protocol: Option<String>,

    #[serde(rename = "rtmp_real_time")]
    pub rtmp_real_time: Option<bool>,
}

// ============================================================================
// 6. Supporting Structs
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedHLSAES {
    #[serde(rename = "uri")]
    pub uri: Option<String>,
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(rename = "iv")]
    pub iv: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFragment {
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    #[serde(rename = "filesize")]
    pub filesize: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedSubtitle {
    #[serde(rename = "ext")]
    pub extension: Option<String>,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "http_headers")]
    pub http_headers: Option<HashMap<String, String>>,
    #[serde(rename = "impersonate")]
    pub impersonate: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedThumbnail {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "ext")]
    pub extension: Option<String>,
    #[serde(rename = "preference")]
    pub preference: Option<i32>,
    #[serde(rename = "width")]
    pub width: Option<i32>,
    #[serde(rename = "height")]
    pub height: Option<i32>,
    #[serde(rename = "resolution")]
    pub resolution: Option<String>,
    #[serde(rename = "filesize")]
    pub filesize: Option<i64>,
    #[serde(rename = "http_headers")]
    pub http_headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedVersion {
    #[serde(rename = "current_git_head")]
    pub current_git_head: Option<String>,
    #[serde(rename = "release_git_head")]
    pub release_git_head: Option<String>,
    #[serde(rename = "repository")]
    pub repository: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedChapterData {
    #[serde(rename = "start_time")]
    pub start_time: Option<f64>,
    #[serde(rename = "end_time")]
    pub end_time: Option<f64>,
    #[serde(rename = "title")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedHeatmapData {
    #[serde(rename = "start_time")]
    pub start_time: Option<f64>,
    #[serde(rename = "end_time")]
    pub end_time: Option<f64>,
    #[serde(rename = "value")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedVideoComment {
    #[serde(rename = "author")]
    pub author: Option<String>,
    #[serde(rename = "author_id")]
    pub author_id: Option<String>,
    #[serde(rename = "author_thumbnail")]
    pub author_thumbnail: Option<String>,
    #[serde(rename = "author_url")]
    pub author_url: Option<String>,
    #[serde(rename = "author_is_verified")]
    pub author_is_verified: Option<bool>,
    #[serde(rename = "author_is_uploader")]
    pub author_is_uploader: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "html")]
    pub html: Option<String>,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: Option<f64>,
    #[serde(rename = "parent")]
    pub parent: Option<String>,
    #[serde(rename = "like_count")]
    pub like_count: Option<f64>,
    #[serde(rename = "dislike_count")]
    pub dislike_count: Option<f64>,
    #[serde(rename = "is_favorited")]
    pub is_favorited: Option<bool>,
    #[serde(rename = "is_pinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "start_time")]
    pub start_time: Option<f64>,
    #[serde(rename = "end_time")]
    pub end_time: Option<f64>,
}
