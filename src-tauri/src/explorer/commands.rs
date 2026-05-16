use std::sync::Arc;
use crate::explorer::service::{DirectoryEntries, ExplorerService};

// ToDo: Properly map errors
#[tauri::command]
pub async fn get_directory_entries(
    directory_id: Option<String>,
    state: tauri::State<'_, Arc<ExplorerService>>,
) -> Result<DirectoryEntries, String> {
    state.get_directory_entries(directory_id).await
        .map_err(|error| error.to_string())
}
