use crate::db::local::models::sync_queue_job::{
    SyncQueueAction, SyncQueueEntityType, SyncQueueJob, SyncQueueJobState,
};
use crate::db::service::DBService;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct SyncQueueService {
    db: Arc<DBService>,
}

impl SyncQueueService {
    pub fn new(db: Arc<DBService>) -> Self {
        Self { db }
    }

    pub async fn enqueue_job(
        &self,
        action: SyncQueueAction,
        entity_type: SyncQueueEntityType,
        entity_id: String,
        payload: Option<String>,
    ) -> Result<SyncQueueJob, String> {
        let now = Utc::now().to_rfc3339();
        let job = SyncQueueJob {
            id: Uuid::new_v4().to_string(),
            action,
            entity_type,
            entity_id,
            state: SyncQueueJobState::Pending,
            payload,
            attempts: 0,
            last_error: None,
            created_at: now.clone(),
            updated_at: now,
        };

        self.db
            .save_sync_queue_job(job.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(job)
    }

    pub async fn get_latest_jobs(&self, limit: i64) -> Result<Vec<SyncQueueJob>, String> {
        self.db
            .get_pending_jobs(limit)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn mark_job_done(&self, job_id: &str) -> Result<(), String> {
        let mut job = self
            .db
            .get_sync_queue_job_by_id(job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Sync queue job not found".to_string())?;

        job.state = SyncQueueJobState::Done;
        job.last_error = None;
        job.updated_at = Utc::now().to_rfc3339();

        self.db
            .patch_sync_queue_job(job)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn mark_job_failed(&self, job_id: &str, error: String) -> Result<(), String> {
        let mut job = self
            .db
            .get_sync_queue_job_by_id(job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Sync queue job not found".to_string())?;

        job.state = SyncQueueJobState::Error;
        job.attempts += 1;
        job.last_error = Some(error);
        job.updated_at = Utc::now().to_rfc3339();

        self.db
            .patch_sync_queue_job(job)
            .await
            .map_err(|e| e.to_string())
    }
}
