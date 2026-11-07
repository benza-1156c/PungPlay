use crate::features::youtube::feed::FeedVideo;
use serde_json::Value;

pub fn parse_feed_videos(json: &Value) -> (Vec<FeedVideo>, Option<String>) {
    let mut videos = Vec::new();
    let mut continuation = None;

    walk_json(json, &mut videos, &mut continuation);

    let mut seen = std::collections::HashSet::new();
    videos.retain(|v| seen.insert(v.id.clone()));

    (videos, continuation)
}

fn walk_json(node: &Value, videos: &mut Vec<FeedVideo>, continuation: &mut Option<String>) {
    match node {
        Value::Object(map) => {
            if map.contains_key("adSlotRenderer") || map.contains_key("inFeedAdLayoutRenderer") {
                return;
            }

            if let Some(vr) = map
                .get("videoRenderer")
                .or_else(|| map.get("playlistVideoRenderer"))
                .or_else(|| map.get("gridVideoRenderer"))
            {
                if let Some(item) = parse_video_renderer(vr) {
                    videos.push(item);
                }
                return;
            }

            if let Some(lvm) = map.get("lockupViewModel") {
                if let Some(item) = parse_lockup_view_model(lvm) {
                    videos.push(item);
                }
                return;
            }

            if continuation.is_none() {
                if let Some(token) = map
                    .get("continuationItemRenderer")
                    .and_then(|c| c.pointer("/continuationEndpoint/continuationCommand/token"))
                    .and_then(|v| v.as_str())
                {
                    *continuation = Some(token.to_string());
                }
            }

            for val in map.values() {
                walk_json(val, videos, continuation);
            }
        }
        Value::Array(arr) => {
            for val in arr {
                walk_json(val, videos, continuation);
            }
        }
        _ => {}
    }
}

fn parse_video_renderer(v: &Value) -> Option<FeedVideo> {
    let id = v.get("videoId")?.as_str()?.to_string();
    if id.starts_with("RD") {
        return None;
    }

    let title = extract_text(v.get("title")?)?;
    if title.trim().is_empty() {
        return None;
    }

    let thumbnail = v
        .pointer("/thumbnail/thumbnails")
        .and_then(|t| t.as_array())
        .and_then(|arr| arr.last())
        .and_then(|thumb| thumb.get("url"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string())
        .or_else(|| Some(format!("https://i.ytimg.com/vi/{id}/hq720.jpg")));

    let channel = v
        .get("ownerText")
        .or_else(|| v.get("longBylineText"))
        .or_else(|| v.get("shortBylineText"))
        .and_then(extract_text);

    let channel_avatar = v
        .pointer("/channelThumbnailSupportedRenderers/channelThumbnailWithLinkRenderer/thumbnail/thumbnails/0/url")
        .or_else(|| v.pointer("/channelThumbnail/thumbnails/0/url"))
        .or_else(|| v.pointer("/ownerText/runs/0/navigationEndpoint/browseEndpoint/avatar/thumbnails/0/url"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    let view_text = v
        .get("viewCountText")
        .and_then(extract_text)
        .or_else(|| v.get("shortViewCountText").and_then(extract_text));

    let view_count = view_text.as_deref().and_then(parse_approx_number);
    let published_time = v.get("publishedTimeText").and_then(extract_text);

    let duration_text = v.get("lengthText").and_then(extract_text).or_else(|| {
        v.pointer("/thumbnailOverlays")
            .and_then(|arr| arr.as_array())
            .and_then(|arr| {
                arr.iter().find_map(|ov| {
                    ov.pointer("/thumbnailOverlayTimeStatusRenderer/text")
                        .and_then(extract_text)
                })
            })
    });
    let duration = duration_text.as_deref().and_then(parse_duration_seconds);

    Some(FeedVideo {
        id: id.clone(),
        title,
        url: format!("https://www.youtube.com/watch?v={id}"),
        duration,
        duration_text,
        channel,
        channel_avatar,
        view_count,
        view_count_text: view_text,
        published_time,
        thumbnail,
    })
}

fn parse_lockup_view_model(v: &Value) -> Option<FeedVideo> {
    let content_type = v.get("contentType").and_then(|c| c.as_str()).unwrap_or("");
    if content_type != "LOCKUP_CONTENT_TYPE_VIDEO" && !content_type.is_empty() {
        return None;
    }

    let id = v.get("contentId")?.as_str()?.to_string();
    if id.starts_with("RD") {
        return None;
    }

    let meta = v.pointer("/metadata/lockupMetadataViewModel")?;
    let title = meta
        .pointer("/title/content")
        .and_then(|t| t.as_str())?
        .trim();

    if title.is_empty() {
        return None;
    }

    let thumbnail = v
        .pointer("/contentImage/thumbnailViewModel/image/sources")
        .and_then(|s| s.as_array())
        .and_then(|arr| arr.last())
        .and_then(|img| img.get("url"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string())
        .or_else(|| Some(format!("https://i.ytimg.com/vi/{id}/hq720.jpg")));

    let channel_avatar = extract_avatar_from_metadata(meta);

    let mut channel = None;
    let mut view_count_text = None;
    let mut published_time = None;

    if let Some(rows) = meta
        .pointer("/metadata/contentMetadataViewModel/metadataRows")
        .and_then(|r| r.as_array())
    {
        if let Some(parts) = rows
            .first()
            .and_then(|r| r.get("metadataParts"))
            .and_then(|p| p.as_array())
        {
            channel = parts
                .first()
                .and_then(|p| p.pointer("/text/content"))
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());
        }
        if let Some(parts) = rows
            .get(1)
            .and_then(|r| r.get("metadataParts"))
            .and_then(|p| p.as_array())
        {
            view_count_text = parts
                .first()
                .and_then(|p| p.pointer("/text/content"))
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());
            published_time = parts
                .get(1)
                .and_then(|p| p.pointer("/text/content"))
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());
        }
    }

    let view_count = view_count_text.as_deref().and_then(parse_approx_number);

    let duration_text = extract_duration_from_lockup(v);
    let duration = duration_text.as_deref().and_then(parse_duration_seconds);

    Some(FeedVideo {
        id: id.clone(),
        title: title.to_string(),
        url: format!("https://www.youtube.com/watch?v={id}"),
        duration,
        duration_text,
        channel,
        channel_avatar,
        view_count,
        view_count_text,
        published_time,
        thumbnail,
    })
}

fn extract_avatar_from_metadata(meta: &Value) -> Option<String> {
    let image = meta.get("image")?;

    // 1. Single decorated avatar
    if let Some(url) = image
        .pointer("/decoratedAvatarViewModel/avatar/avatarViewModel/image/sources/0/url")
        .or_else(|| image.pointer("/decoratedAvatarViewModel/avatarViewModel/image/sources/0/url"))
        .and_then(|u| u.as_str())
    {
        return Some(url.to_string());
    }

    // 2. Avatar stack (collaboration videos with multiple artists)
    if let Some(url) = image
        .pointer("/avatarStackViewModel/avatars/0/avatarViewModel/image/sources/0/url")
        .and_then(|u| u.as_str())
    {
        return Some(url.to_string());
    }

    None
}

fn extract_duration_from_lockup(v: &Value) -> Option<String> {
    let overlays = v
        .pointer("/contentImage/thumbnailViewModel/overlays")?
        .as_array()?;

    for ov in overlays {
        // 1. thumbnailBottomOverlayViewModel
        if let Some(badges) = ov
            .pointer("/thumbnailBottomOverlayViewModel/badges")
            .and_then(|b| b.as_array())
        {
            for badge in badges {
                if let Some(text) = badge
                    .pointer("/thumbnailBadgeViewModel/text")
                    .and_then(|t| t.as_str())
                {
                    return Some(text.to_string());
                }
            }
        }

        // 2. thumbnailOverlayBadgeViewModel
        if let Some(badges) = ov
            .pointer("/thumbnailOverlayBadgeViewModel/thumbnailBadges")
            .and_then(|b| b.as_array())
        {
            for badge in badges {
                if let Some(text) = badge
                    .pointer("/thumbnailBadgeViewModel/text")
                    .and_then(|t| t.as_str())
                {
                    return Some(text.to_string());
                }
            }
        }
    }

    None
}

fn extract_text(node: &Value) -> Option<String> {
    if let Some(simple) = node.get("simpleText").and_then(|s| s.as_str()) {
        return Some(simple.to_string());
    }
    if let Some(runs) = node.get("runs").and_then(|r| r.as_array()) {
        let joined: String = runs
            .iter()
            .filter_map(|r| r.get("text").and_then(|t| t.as_str()))
            .collect();
        if !joined.is_empty() {
            return Some(joined);
        }
    }
    None
}

fn parse_duration_seconds(text: &str) -> Option<f64> {
    let parts: Vec<f64> = text
        .trim()
        .split(':')
        .filter_map(|p| p.trim().parse::<f64>().ok())
        .collect();
    match parts.as_slice() {
        [s] => Some(*s),
        [m, s] => Some(m * 60.0 + s),
        [h, m, s] => Some(h * 3600.0 + m * 60.0 + s),
        [d, h, m, s] => Some(d * 86400.0 + h * 3600.0 + m * 60.0 + s),
        _ => None,
    }
}

fn parse_approx_number(text: &str) -> Option<f64> {
    let cleaned = text.replace(',', "");
    let mut num_str = String::new();
    for ch in cleaned.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            num_str.push(ch);
        } else if !num_str.is_empty() {
            break;
        }
    }
    let base: f64 = num_str.parse().ok()?;

    let mult = if cleaned.contains('B') || cleaned.contains('b') || cleaned.contains("พันล้าน")
    {
        1_000_000_000.0
    } else if cleaned.contains('M') || cleaned.contains('m') || cleaned.contains("ล้าน") {
        1_000_000.0
    } else if cleaned.contains("แสน") {
        100_000.0
    } else if cleaned.contains("หมื่น") {
        10_000.0
    } else if cleaned.contains('K') || cleaned.contains('k') || cleaned.contains("พัน") {
        1_000.0
    } else {
        1.0
    };

    Some((base * mult).round())
}
