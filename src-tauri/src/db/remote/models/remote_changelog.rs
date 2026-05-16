use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[derive(sqlx::Type, strum_macros::Display)]
pub enum EventType {
    CREATE,
    RENAME,
    MOVE,
    DELETE,
    EDIT
}

#[derive(Deserialize, Debug)]
#[derive(sqlx::Type, strum_macros::Display)]
pub enum EntityType {
    DIRECTORY,
    FILE
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteChangelog {
    pub id: i64,
    pub event_type: EventType,
    pub entity_type: EntityType,
    pub timestamp: String,
    pub file_id: Option<String>,
    pub folder_id: Option<String>,
    pub  user_id: String,
}
