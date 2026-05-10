use std::sync::Arc;

use crate::fs::service::{FilesystemService};

#[tauri::command]
pub async fn receive_dropped_paths(
    paths: Vec<String>,
    state: tauri::State<'_, Arc<FilesystemService>>,
) -> Result<(), String> {
    state.write_dropped_paths(paths, None).await
}
