use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use crate::api::service::ApiService;
use crate::db::service::{Changelog, DBService, Status};

pub struct SyncService {
    api: Arc<ApiService>,
    db: Arc<DBService>
}

impl SyncService {
    pub fn new(api: Arc<ApiService>, db: Arc<DBService>) -> SyncService {
        Self { api, db }
    }


    pub async fn run_sync(&self, app: AppHandle) {
        self.sync_changelogs(app).await;
    }

    // ToDo: Probably lock the changelog table?
    // ToDo: Remove expect
    async fn sync_changelogs(&self, app: AppHandle) {
        // Retrieve latest changelogs id
        let id = self.db.get_latest_changelog_id().await.expect("Failed to get latest changelog id");
        println!("Latest changelog id: {}", id);

        // Fetch remote changelogs with latest id
        // ToDo: Add pagination
        let changelogs: Vec<Changelog> = self.api.get_latest_changelogs(id).await.expect("Failed to get latest changelogs");
        println!("Current changelog count: {}", changelogs.len());

        // Insert into db
        // ToDo: Might implement batching
        for changelog in changelogs {
            let changelog: Status = self.db.save_changelog(changelog).await.expect("Failed to save changelog");
            println!("Notfying frontend... {:?}", changelog);
            // Notify frontend
            app.emit("synch-changelogs", changelog).expect("Failed to emit changelog");
        }


    }

}