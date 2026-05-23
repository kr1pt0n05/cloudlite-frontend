use crate::explorer::service::{DirectoryEntries, ExplorerService};
use std::sync::Arc;
use tauri::AppHandle;

// ToDo: Properly map errors
#[tauri::command]
pub async fn get_directory_entries(
    directory_id: Option<String>,
    state: tauri::State<'_, Arc<ExplorerService>>,
) -> Result<DirectoryEntries, String> {
    state
        .get_directory_entries(directory_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn rename_directory(
    directory_id: String,
    name: String,
    state: tauri::State<'_, Arc<ExplorerService>>,
) -> Result<String, String> {
    state.rename_directory(directory_id, name).await
}

#[tauri::command]
pub async fn rename_file(
    file_id: String,
    filename: String,
    state: tauri::State<'_, Arc<ExplorerService>>,
) -> Result<String, String> {
    state.rename_file(file_id, filename).await
}

#[tauri::command]
pub async fn receive_dropped_paths(
    paths: Vec<String>,
    destination_path: Option<String>,
    app: AppHandle,
    state: tauri::State<'_, Arc<ExplorerService>>,
) -> Result<(), String> {
    state
        .write_dropped_paths(app, paths, destination_path)
        .await
}
