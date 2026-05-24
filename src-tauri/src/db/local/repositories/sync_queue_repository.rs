use crate::db::local::models::sync_queue_job::{
    SyncQueueAction, SyncQueueEntityType, SyncQueueJob, SyncQueueJobState,
};
use crate::db::service::DBService;

impl DBService {
    pub async fn create_sync_queue_if_not_exists(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS sync_queue_jobs (
                id TEXT PRIMARY KEY NOT NULL,
                action TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                state TEXT NOT NULL,
                payload TEXT,
                attempts INTEGER NOT NULL DEFAULT 0,
                last_error TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );"
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_sync_queue_job(&self, job: SyncQueueJob) -> Result<(), sqlx::Error> {
        let action = job.action.as_str();
        let entity_type = job.entity_type.as_str();
        let state = job.state.as_str();

        sqlx::query!(
            "INSERT INTO sync_queue_jobs (
                id, action, entity_type, entity_id, state, payload, attempts, last_error, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            job.id,
            action,
            entity_type,
            job.entity_id,
            state,
            job.payload,
            job.attempts,
            job.last_error,
            job.created_at,
            job.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_sync_queue_job_by_id(
        &self,
        id: &str,
    ) -> Result<Option<SyncQueueJob>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                id AS "id!",
                action AS "action!",
                entity_type AS "entity_type!",
                entity_id AS "entity_id!",
                state AS "state!",
                payload AS "payload?",
                attempts AS "attempts!",
                last_error AS "last_error?",
                created_at AS "created_at!",
                updated_at AS "updated_at!"
            FROM sync_queue_jobs
            WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        row.map(|row| {
            sync_queue_job_from_columns(
                row.id,
                row.action,
                row.entity_type,
                row.entity_id,
                row.state,
                row.payload,
                row.attempts,
                row.last_error,
                row.created_at,
                row.updated_at,
            )
        })
        .transpose()
    }

    pub async fn get_pending_jobs(&self, limit: i64) -> Result<Vec<SyncQueueJob>, sqlx::Error> {
        let state = SyncQueueJobState::Pending.as_str();

        let rows = sqlx::query!(
            r#"
            SELECT
                id AS "id!",
                action AS "action!",
                entity_type AS "entity_type!",
                entity_id AS "entity_id!",
                state AS "state!",
                payload AS "payload?",
                attempts AS "attempts!",
                last_error AS "last_error?",
                created_at AS "created_at!",
                updated_at AS "updated_at!"
            FROM sync_queue_jobs
            WHERE state = ?1
            ORDER BY created_at ASC
            LIMIT ?2
            "#,
            state,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                sync_queue_job_from_columns(
                    row.id,
                    row.action,
                    row.entity_type,
                    row.entity_id,
                    row.state,
                    row.payload,
                    row.attempts,
                    row.last_error,
                    row.created_at,
                    row.updated_at,
                )
            })
            .collect()
    }

    pub async fn patch_sync_queue_job(&self, job: SyncQueueJob) -> Result<(), sqlx::Error> {
        let action = job.action.as_str();
        let entity_type = job.entity_type.as_str();
        let state = job.state.as_str();

        sqlx::query!(
            "UPDATE sync_queue_jobs
            SET action = COALESCE(?1, action),
                entity_type = COALESCE(?2, entity_type),
                entity_id = COALESCE(?3, entity_id),
                state = COALESCE(?4, state),
                payload = ?5,
                attempts = COALESCE(?6, attempts),
                last_error = ?7,
                updated_at = COALESCE(?8, updated_at)
            WHERE id = ?9",
            action,
            entity_type,
            job.entity_id,
            state,
            job.payload,
            job.attempts,
            job.last_error,
            job.updated_at,
            job.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_sync_queue_job(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM sync_queue_jobs
            WHERE id = ?1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub(crate) async fn drop_sync_queue_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query!("DROP TABLE IF EXISTS sync_queue_jobs;")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

fn sync_queue_job_from_columns(
    id: String,
    action: String,
    entity_type: String,
    entity_id: String,
    state: String,
    payload: Option<String>,
    attempts: i64,
    last_error: Option<String>,
    created_at: String,
    updated_at: String,
) -> Result<SyncQueueJob, sqlx::Error> {
    Ok(SyncQueueJob {
        id,
        action: SyncQueueAction::from_db(&action).map_err(sqlx::Error::Protocol)?,
        entity_type: SyncQueueEntityType::from_db(&entity_type).map_err(sqlx::Error::Protocol)?,
        entity_id,
        state: SyncQueueJobState::from_db(&state).map_err(sqlx::Error::Protocol)?,
        payload,
        attempts,
        last_error,
        created_at,
        updated_at,
    })
}
