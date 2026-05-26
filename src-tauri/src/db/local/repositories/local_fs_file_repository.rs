use crate::db::local::models::local_fs_file::LocalFsFile;
use crate::db::service::DBService;
use std::path::PathBuf;

impl DBService {
    pub async fn create_local_fs_file_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS local_fs_files (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                directory TEXT,
                checksum TEXT,
                size INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT,

                FOREIGN KEY (directory)
                    REFERENCES local_fs_directories(id)
                    ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_local_fs_files_directory ON local_fs_files(directory);
            "
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_file_by_id(&self, id: &str) -> Result<Option<LocalFsFile>, sqlx::Error> {
        sqlx::query_as!(
            LocalFsFile,
            r#"
        SELECT
            id as "id!",
            name,
            directory,
            checksum,
            size,
            mime_type,
            created_at,
            updated_at
        FROM local_fs_files
        WHERE id = $1
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_local_file_path_by_file_id(
        &self,
        id: &str,
    ) -> Result<Option<PathBuf>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT directory
            FROM local_fs_files
            WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(directory_id) = row.and_then(|row| row.directory) {
            return self.get_local_directory_path_by_id(&directory_id).await;
        }

        Ok(Some(PathBuf::new()))
    }

    pub async fn patch_local_file(&self, updated_file: LocalFsFile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE local_fs_files
            SET name = COALESCE(?1, name),
                directory = COALESCE(?2, directory),
                checksum = COALESCE(?3, checksum),
                size = COALESCE(?4, size),
                mime_type = COALESCE(?5, mime_type),
                updated_at = ?6
            WHERE id = ?7",
            updated_file.name,
            updated_file.directory,
            updated_file.checksum,
            updated_file.size,
            updated_file.mime_type,
            updated_file.updated_at,
            updated_file.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_local_file(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM local_fs_files
            WHERE id = ?1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_local_file(&self, file: LocalFsFile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO local_fs_files (id, name, directory, checksum, size, mime_type, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            file.id,
            file.name,
            file.directory,
            file.checksum,
            file.size,
            file.mime_type,
            file.created_at
        )
        .execute(&self.pool)
        .await?;
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

    pub(crate) async fn drop_local_files_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query!("DROP TABLE IF EXISTS local_fs_files;")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
