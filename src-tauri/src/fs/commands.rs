use std::sync::Arc;

use crate::fs::service::{DroppedFsEntry, FilesystemService};

#[tauri::command]
pub fn receive_dropped_paths(
    paths: Vec<String>,
    state: tauri::State<'_, Arc<FilesystemService>>,
) -> Result<Vec<DroppedFsEntry>, String> {
    state.collect_dropped_entries(paths)
}
