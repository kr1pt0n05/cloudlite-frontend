use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::service::DBService;
use uuid::Uuid;

impl DBService {
    pub async fn create_local_fs_directory_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS local_fs_directories(
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                parent TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT,

                FOREIGN KEY (parent)
                    REFERENCES local_fs_directories(id)
                    ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_local_fs_directories_parent ON local_fs_directories(parent);
            "
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn save_local_directory(
        &self,
        directory: LocalFsDirectory,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO local_fs_directories (id, name, path, parent, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            directory.id,
            directory.name,
            directory.path,
            directory.parent,
            directory.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_directory_by_path(&self, path: &str) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query!("SELECT id FROM local_fs_directories WHERE path = ?1", path)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.id).flatten())
    }

    pub async fn get_subdirectories(
        &self,
        parent_id: &Option<String>,
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

    pub(crate) async fn drop_local_fs_directory_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query!("DROP TABLE IF EXISTS local_fs_directories;")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
