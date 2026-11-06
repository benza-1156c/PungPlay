use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "action", content = "payload", rename_all = "snake_case")]
pub enum AuthEvent {
    Success { cookie: String },
    Closed,
    Error(String),
}

