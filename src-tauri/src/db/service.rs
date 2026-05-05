use sqlx::sqlite::SqlitePoolOptions;

#[derive(sqlx::Type, strum_macros::Display)]
enum EventType {
    CREATE,
    RENAME,
    MOVE,
    DELETE,
    EDIT
}

#[derive(sqlx::Type, strum_macros::Display)]
enum EntityType {
    DIRECTORY,
    FILE
}

struct Status {
    id: i64,
    success: bool,
    error: Option<String>,
}

pub struct Changelog {
    id: i64,
    event_type: EventType,
    entity_type: EntityType,
    timestamp: String,
    file_id: String,
    folder_id: String,
    user_id: String,
}


pub struct DBService{
    pool: sqlx::SqlitePool,
}

impl DBService {
    pub async fn new() -> Self{
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .min_connections(1)
            .connect("sqlite:./dev.db")
            .await
            .unwrap();
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