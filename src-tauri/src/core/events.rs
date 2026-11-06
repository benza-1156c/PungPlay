use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "action", content = "payload", rename_all = "snake_case")]
pub enum AuthEvent {
    Success { cookie: String },
    Closed,
    Error(String),
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "action", content = "payload", rename_all = "snake_case")]
pub enum YtdlpEvent {
    Progress { downloaded: u32, total: u32 },
    Complete { total: u32 },
    Ready,
    Error(String),
}

#[derive(Serialize, Deserialize, Type, Event, Clone, Debug)]
#[serde(tag = "domain", content = "data", rename_all = "snake_case")]
pub enum AppEvent {
    Auth(AuthEvent),
    Ytdlp(YtdlpEvent),
}

impl AuthEvent {
    pub fn emit_event(self, app: &tauri::AppHandle) {
        let _ = AppEvent::Auth(self).emit(app);
    }
}

impl YtdlpEvent {
    pub fn emit_event(self, app: &tauri::AppHandle) {
        let _ = AppEvent::Ytdlp(self).emit(app);
    }
}
