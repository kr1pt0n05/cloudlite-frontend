use std::sync::Arc;
use tauri::AppHandle;
use crate::sync::service::SyncService;

#[tauri::command]
pub async fn run_sync(app: AppHandle, state: tauri::State<'_, Arc<SyncService>>) -> Result<(), String> {
    state.run_sync(app).await;
    Ok(())
}
