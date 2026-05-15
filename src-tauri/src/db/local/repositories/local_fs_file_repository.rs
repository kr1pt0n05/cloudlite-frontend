use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::local::models::local_fs_file::LocalFsFile;
use crate::db::service::DBService;

impl DBService {

    pub async fn create_local_fs_file_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS local_fs_files (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                directory TEXT,
                checksum TEXT,
                size INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT
            );"
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_files_of_directory(
        &self,
        directory_id: &Option<String>,
    ) -> sqlx::Result<Vec<LocalFsFile>> {
        sqlx::query_as!(
        LocalFsFile,
        r#"
        SELECT
            id AS "id!",
            name AS "name!",
            directory AS "directory?",
            checksum AS "checksum?",
            size as "size!",
            mime_type as "mime_type!",
            created_at AS "created_at!",
            updated_at AS "updated_at?"
        FROM local_fs_files
        WHERE directory = ?1 OR (?1 IS NULL AND directory IS NULL)
        "#,
        directory_id
    )
            .fetch_all(&self.pool)
            .await
    }

}