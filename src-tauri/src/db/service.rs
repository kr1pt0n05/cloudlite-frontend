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


}
