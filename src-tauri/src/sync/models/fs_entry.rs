use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub enum SyncState {
    Pending,
    Syncing,
    Synced,
    Error(String),
}

#[derive(Serialize, Deserialize)]
enum FsEntryKind {
    File,
    Directory,
}
#[derive(Serialize, Deserialize)]
pub struct FsEntry {
    pub id: Option<String>,
    pub name: String,
    pub size: Option<u64>,
    pub state: SyncState,
    pub kind: FsEntryKind,
    pub modified: Option<String>,
}