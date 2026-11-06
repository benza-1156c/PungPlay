use crate::features::auth::entity as profile_entity;
use sea_orm::{
    sea_query::TableCreateStatement, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    Schema,
};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, Manager};

pub fn get_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app_local_data_dir: {e}"))?;

    fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create app data directory: {e}"))?;

    Ok(app_dir.join("pungplay.db"))
}

pub async fn init_db(app: &AppHandle) -> Result<DatabaseConnection, String> {
    let db_path = get_db_path(app)?;
    let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .map_err(|e| format!("Failed to connect to SQLite database: {e}"))?;

    db.execute_unprepared("PRAGMA journal_mode = WAL;")
        .await
        .map_err(|e| format!("Failed to set WAL mode: {e}"))?;
    db.execute_unprepared("PRAGMA busy_timeout = 5000;")
        .await
        .map_err(|e| format!("Failed to set busy timeout: {e}"))?;

    create_tables(&db).await?;

    Ok(db)
}

async fn create_tables(db: &DatabaseConnection) -> Result<(), String> {
    let _ = db.execute_unprepared("DROP TABLE IF EXISTS cookies;").await;

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let table: TableCreateStatement = schema
        .create_table_from_entity(profile_entity::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(&table)
        .await
        .map_err(|e| format!("Failed to create profiles table: {e}"))?;

    Ok(())
}
