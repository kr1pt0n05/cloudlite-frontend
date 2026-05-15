use std::sync::Arc;
use tauri::{AppHandle, Error};
use crate::explorer::service::{DirectoryEntries, ExplorerService};

// ToDo: Properly map errors
#[tauri::command]
pub async fn get_directory_entries(app: AppHandle, state: tauri::State<'_, Arc<ExplorerService>>, directory_id: Option<String>) -> Result<DirectoryEntries, sqlx::Error> {
    state.get_directory_entries(directory_id).await
}