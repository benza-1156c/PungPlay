use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

use super::auth::AuthEvent;

#[derive(Serialize, Deserialize, Type, Event, Clone, Debug)]
#[serde(tag = "domain", content = "data", rename_all = "snake_case")]
pub enum AppEvent {
    Auth(AuthEvent),
    // Player(PlayerEvent),
}

impl AuthEvent {
    pub fn emit_event(self, app: &tauri::AppHandle) {
        let _ = AppEvent::Auth(self).emit(app);
    }
}
