use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Type)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Video,
    Playlist,
    Channel,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Type)]
pub struct SearchResultItem {
    pub item_type: ItemType,
    pub id: String,
    pub title: String,
    pub thumbnail: String,
    pub url: String,
}
