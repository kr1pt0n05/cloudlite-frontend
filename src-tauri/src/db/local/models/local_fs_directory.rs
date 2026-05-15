use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalFsDirectory {
    pub id: String,
    pub name: String,
    pub path: String,
    pub parent: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}