use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
pub(crate) use crate::db::models::changelog::Changelog;

#[derive(Clone, Serialize, Debug)]
pub struct Status {
    id: i64,
    success: bool,
    error: Option<String>,
}

#[derive(Clone)]
pub struct DBService{
    pub(crate) pool: sqlx::SqlitePool,
}

impl DBService {
    pub async fn new() -> Self{
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .min_connections(1)
            .connect("sqlite:./dev.db")
            .await.expect("Failed to connect to database");
        Self { pool }
    }



    pub async fn create_changelogs_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS changelogs (
                id INTEGER PRIMARY KEY,
                event_type VARCHAR(10) NOT NULL,
                entity_type VARCHAR(10) NOT NULL,
                timestamp TEXT NOT NULL,
                file_id TEXT,
                folder_id TEXT,
                user_id TEXT NOT NULL
            );"
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_latest_changelog_id(&self) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT id FROM changelogs ORDER BY id DESC LIMIT 1"
        ).fetch_optional(&self.pool)
        .await?;
        println!("Latest changelog id from db: {:?}", result);
        Ok(result.map_or(0, |row| row.id))
    }

    // ToDo: Might implement batch inserting
    pub async fn save_changelog(&self, log: Changelog) -> Result<Status, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO changelogs (id, event_type, entity_type, timestamp, file_id, folder_id, user_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            log.id,
            log.event_type,
            log.entity_type,
            log.timestamp,
            log.file_id,
            log.folder_id,
            log.user_id
        ).execute(&self.pool)
            .await;

        let status = match result {
            Ok(_) => Status { id: log.id, success: true, error: None },
            Err(e) => Status {
                id: log.id,
                success: false,
                error: Some(e.to_string()),
            },
        };
        Ok(status)
    }


}
