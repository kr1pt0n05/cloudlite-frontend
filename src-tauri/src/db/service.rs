use serde::{Serialize};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Clone, Serialize, Debug)]
pub struct Status {
    pub(crate) id: i64,
    pub(crate) success: bool,
    pub(crate) error: Option<String>,
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

    pub async fn drop_all_tables(&self) {
        self.drop_sync_queue_table().await.expect("Failed to drop sync_queue_jobs table");
        self.drop_local_fs_directory_table().await.expect("Failed to drop local_fs_directory table");
        self.drop_local_files_table().await.expect("Failed to drop local_fs_file table");
    }

}
