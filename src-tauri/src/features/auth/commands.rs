use crate::core::events::AuthEvent;
use crate::features::auth::window::{get_or_create_login_window, spawn_login_watcher};
use tauri::AppHandle;

#[derive(serde::Serialize, serde::Deserialize, specta::Type, Clone, Debug)]
pub struct ProfileSummary {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
}

#[tauri::command]
#[specta::specta]
pub async fn login_youtube(app: AppHandle) -> Result<(), String> {
    match get_or_create_login_window(&app) {
        Ok(_) => {
            spawn_login_watcher(app);
            Ok(())
        }
        Err(e) => {
            AuthEvent::Error(e.clone()).emit_event(&app);
            Err(e)
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn logout_youtube(
    db: tauri::State<'_, sea_orm::DatabaseConnection>,
) -> Result<(), String> {
    crate::features::auth::repository::clear_all_profiles(&db).await
}

#[tauri::command]
#[specta::specta]
pub async fn is_authenticated(
    db: tauri::State<'_, sea_orm::DatabaseConnection>,
) -> Result<bool, String> {
    crate::features::auth::repository::has_active_auth(&db).await
}

#[tauri::command]
#[specta::specta]
pub async fn list_profiles(
    db: tauri::State<'_, sea_orm::DatabaseConnection>,
) -> Result<Vec<ProfileSummary>, String> {
    let profiles = crate::features::auth::repository::list_profiles(&db).await?;
    Ok(profiles
        .into_iter()
        .map(|p| ProfileSummary {
            id: p.id,
            name: p.name,
            email: p.email,
            avatar_url: p.avatar_url,
            is_active: p.is_active,
        })
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn switch_profile(
    db: tauri::State<'_, sea_orm::DatabaseConnection>,
    profile_id: String,
) -> Result<(), String> {
    crate::features::auth::repository::switch_profile(&db, &profile_id).await
}

