use crate::features::auth::entity::{self, Column, CookieItem, Entity as Profile};
use chrono::Utc;
use sea_orm::{
    sea_query::OnConflict, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use tauri::webview::Cookie as TauriCookie;

pub fn to_cookie_items(cookies: &[TauriCookie<'_>]) -> Vec<CookieItem> {
    cookies
        .iter()
        .map(|c| CookieItem {
            name: c.name().to_string(),
            value: c.value().to_string(),
            domain: c.domain().unwrap_or("").to_string(),
            path: c.path().unwrap_or("/").to_string(),
            expires: c.expires().map(|exp| format!("{exp:?}")),
            http_only: c.http_only().unwrap_or(false),
            secure: c.secure().unwrap_or(false),
        })
        .collect()
}

pub fn build_cookie_header(cookies: &[TauriCookie<'_>]) -> String {
    let mut yt_cookies = std::collections::BTreeMap::new();

    for c in cookies {
        let domain = c.domain().unwrap_or("");
        if domain.contains("youtube.com") {
            yt_cookies.insert(c.name().to_string(), c.value().to_string());
        }
    }

    if yt_cookies.is_empty() {
        for c in cookies {
            yt_cookies.insert(c.name().to_string(), c.value().to_string());
        }
    }

    yt_cookies
        .into_iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<String>>()
        .join("; ")
}

pub fn build_google_cookie_header(cookies: &[TauriCookie<'_>]) -> String {
    let mut g_cookies = std::collections::BTreeMap::new();

    for c in cookies {
        let domain = c.domain().unwrap_or("");
        if domain.contains("google.com") && !domain.contains("youtube") {
            g_cookies.insert(c.name().to_string(), c.value().to_string());
        }
    }

    g_cookies
        .into_iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<String>>()
        .join("; ")
}

pub async fn save_profile_cookies(
    db: &DatabaseConnection,
    profile_id: &str,
    profile_name: &str,
    email: Option<String>,
    avatar_url: Option<String>,
    cookies: &[TauriCookie<'_>],
) -> Result<String, String> {
    let cookie_header = build_cookie_header(cookies);
    let cookie_items = to_cookie_items(cookies);
    let now = Utc::now();

    Profile::update_many()
        .col_expr(Column::IsActive, sea_orm::sea_query::Expr::value(false))
        .exec(db)
        .await
        .map_err(|e| format!("Failed to deactivate other profiles: {e}"))?;

    let active_model = entity::ActiveModel {
        id: Set(profile_id.to_string()),
        name: Set(profile_name.to_string()),
        email: Set(email),
        avatar_url: Set(avatar_url),
        cookie_header: Set(cookie_header.clone()),
        cookies: Set(cookie_items.into()),
        is_active: Set(true),
        updated_at: Set(now),
    };

    Profile::insert(active_model)
        .on_conflict(
            OnConflict::column(Column::Id)
                .update_columns([
                    Column::Name,
                    Column::Email,
                    Column::AvatarUrl,
                    Column::CookieHeader,
                    Column::Cookies,
                    Column::IsActive,
                    Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec(db)
        .await
        .map_err(|e| format!("Failed to upsert profile: {e}"))?;

    Ok(cookie_header)
}

pub async fn get_active_profile(db: &DatabaseConnection) -> Result<Option<entity::Model>, String> {
    Profile::find()
        .filter(Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| format!("Failed to query active profile: {e}"))
}

pub async fn get_active_cookie_header(db: &DatabaseConnection) -> Result<String, String> {
    let profile = get_active_profile(db).await?;
    match profile {
        Some(p) => Ok(p.cookie_header),
        None => Ok(String::new()),
    }
}

pub async fn has_active_auth(db: &DatabaseConnection) -> Result<bool, String> {
    let profile = get_active_profile(db).await?;
    if let Some(p) = profile {
        let has_tokens = p.cookie_header.contains("SAPISID")
            || p.cookie_header.contains("SID")
            || p.cookie_header.contains("HSID")
            || p.cookie_header.contains("LOGIN_INFO");
        Ok(has_tokens)
    } else {
        Ok(false)
    }
}

pub async fn list_profiles(db: &DatabaseConnection) -> Result<Vec<entity::Model>, String> {
    Profile::find()
        .all(db)
        .await
        .map_err(|e| format!("Failed to list profiles: {e}"))
}

pub async fn switch_profile(db: &DatabaseConnection, profile_id: &str) -> Result<(), String> {
    // Deactivate all
    Profile::update_many()
        .col_expr(Column::IsActive, sea_orm::sea_query::Expr::value(false))
        .exec(db)
        .await
        .map_err(|e| format!("Failed to deactivate profiles: {e}"))?;

    // Activate the selected profile
    Profile::update_many()
        .filter(Column::Id.eq(profile_id))
        .col_expr(Column::IsActive, sea_orm::sea_query::Expr::value(true))
        .exec(db)
        .await
        .map_err(|e| format!("Failed to activate profile {profile_id}: {e}"))?;

    Ok(())
}

/// Delete a profile by ID.
pub async fn delete_profile(db: &DatabaseConnection, profile_id: &str) -> Result<(), String> {
    Profile::delete_many()
        .filter(Column::Id.eq(profile_id))
        .exec(db)
        .await
        .map_err(|e| format!("Failed to delete profile {profile_id}: {e}"))?;

    Ok(())
}

pub async fn logout_youtube(db: &DatabaseConnection) -> Result<(), String> {
    Profile::update_many()
        .col_expr(Column::IsActive, false.into())
        .filter(Column::IsActive.eq(true))
        .exec(db)
        .await
        .map_err(|e| format!("Failed to clear all profiles: {e}"))?;
    Ok(())
}
