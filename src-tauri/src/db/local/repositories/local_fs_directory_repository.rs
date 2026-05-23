use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::service::DBService;
use std::path::PathBuf;

impl DBService {
    pub async fn create_local_fs_directory_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS local_fs_directories(
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT,

                FOREIGN KEY (parent)
                    REFERENCES local_fs_directories(id)
                    ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_local_fs_directories_parent ON local_fs_directories(parent);
            "
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_local_directory_by_id(
        &self,
        id: &str,
    ) -> Result<Option<LocalFsDirectory>, sqlx::Error> {
        sqlx::query_as!(
            LocalFsDirectory,
            r#"
            SELECT
                id as "id!",
                name,
                parent,
                created_at,
                updated_at
            FROM local_fs_directories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_local_directory_path_by_id(
        &self,
        id: &str,
    ) -> Result<Option<PathBuf>, sqlx::Error> {
        let mut path_segments = Vec::new();
        let mut current_id = Some(id.to_string());

        while let Some(directory_id) = current_id {
            let directory = self.get_local_directory_by_id(&directory_id).await?;

            if let Some(directory) = directory {
                path_segments.push(directory.name);
                current_id = directory.parent;
            } else {
                return Ok(None);
            }
        }

        path_segments.reverse();
        Ok(Some(path_segments.iter().collect()))
    }

    pub async fn patch_local_directory(
        &self,
        updated_directory: LocalFsDirectory,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE local_fs_directories
            SET
                name = COALESCE(?1, name),
                parent = COALESCE(?2, parent),
                created_at = COALESCE(?3, created_at),
                updated_at = COALESCE(?4, updated_at)
            WHERE id = ?5",
            updated_directory.name,
            updated_directory.parent,
            updated_directory.created_at,
            updated_directory.updated_at,
            updated_directory.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_local_directory(
        &self,
        directory: LocalFsDirectory,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO local_fs_directories (id, name, parent, created_at)
            VALUES (?1, ?2, ?3, ?4)",
            directory.id,
            directory.name,
            directory.parent,
            directory.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_local_directory(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM local_fs_directories
            WHERE id = ?1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
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
