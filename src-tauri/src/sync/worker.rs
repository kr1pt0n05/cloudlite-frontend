use crate::db::local::models::sync_queue_job::SyncQueueJob;
use crate::sync::queue::SyncQueueService;
use std::sync::Arc;

const DEFAULT_SYNC_BATCH_SIZE: i64 = 250;

pub struct SyncWorkerService {
    queue: Arc<SyncQueueService>,
    batch_size: i64,
}

impl SyncWorkerService {
    pub fn new(queue: Arc<SyncQueueService>) -> Self {
        Self {
            queue,
            batch_size: DEFAULT_SYNC_BATCH_SIZE,
        }
    }

    pub async fn poll_jobs(&self) -> Result<Vec<SyncQueueJob>, String> {
        self.queue.get_latest_jobs(self.batch_size).await
    }
}
