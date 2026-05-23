use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncQueueAction {
    Create,
    Rename,
    Move,
    Delete,
    Edit,
}

impl SyncQueueAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "CREATE",
            Self::Rename => "RENAME",
            Self::Move => "MOVE",
            Self::Delete => "DELETE",
            Self::Edit => "EDIT",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "CREATE" => Ok(Self::Create),
            "RENAME" => Ok(Self::Rename),
            "MOVE" => Ok(Self::Move),
            "DELETE" => Ok(Self::Delete),
            "EDIT" => Ok(Self::Edit),
            _ => Err(format!("Unknown sync queue action: {}", value)),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncQueueEntityType {
    Directory,
    File,
}

impl SyncQueueEntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Directory => "DIRECTORY",
            Self::File => "FILE",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "DIRECTORY" => Ok(Self::Directory),
            "FILE" => Ok(Self::File),
            _ => Err(format!("Unknown sync queue entity type: {}", value)),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncQueueJobState {
    Pending,
    Synching,
    Done,
    Error,
}

impl SyncQueueJobState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "PENDING",
            Self::Synching => "SYNCHING",
            Self::Done => "DONE",
            Self::Error => "ERROR",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "PENDING" => Ok(Self::Pending),
            "SYNCHING" => Ok(Self::Synching),
            "DONE" => Ok(Self::Done),
            "ERROR" => Ok(Self::Error),
            _ => Err(format!("Unknown sync queue job state: {}", value)),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncQueueJob {
    pub id: String,
    pub action: SyncQueueAction,
    pub entity_type: SyncQueueEntityType,
    pub entity_id: String,
    pub state: SyncQueueJobState,
    pub payload: Option<String>,
    pub attempts: i64,
    pub last_error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
