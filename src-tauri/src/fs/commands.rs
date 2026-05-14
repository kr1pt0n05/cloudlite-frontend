use std::sync::Arc;

use crate::fs::service::FilesystemService;
use tauri::AppHandle;

#[tauri::command]
pub async fn receive_dropped_paths(
    paths: Vec<String>,
    destination_path: Option<String>,
    app: AppHandle,
    state: tauri::State<'_, Arc<FilesystemService>>,
) -> Result<(), String> {
    state
        .write_dropped_paths(app, paths, destination_path)
        .await
}
