use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::service::DBService;

impl DBService {

    pub async fn create_local_fs_directory_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS local_fs_directories(
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                parent TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT
            );"
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_subdirectories(
        &self,
        parent_id: Option<&str>,
    ) -> sqlx::Result<Vec<LocalFsDirectory>> {
        sqlx::query_as!(
        LocalFsDirectory,
        r#"
        SELECT
            id AS "id!",
            name AS "name!",
            path AS "path!",
            parent AS "parent?",
            created_at AS "created_at!",
            updated_at AS "updated_at?"
        FROM local_fs_directories
        WHERE parent = ?1 OR (?1 IS NULL AND parent IS NULL)
        "#,
        parent_id
    )
            .fetch_all(&self.pool)
            .await
    }

}