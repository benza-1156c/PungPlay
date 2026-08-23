use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")] // เวทมนตร์! มันจะแปลง Video -> "video" ให้อัตโนมัติ
pub enum ItemType {
    Video,
    Playlist,
    Channel,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResultItem {
    pub item_type: ItemType,
    pub id: String,
    pub title: String,
    pub thumbnail: String,
    pub url: String,
}
