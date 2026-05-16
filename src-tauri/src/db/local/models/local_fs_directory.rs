use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalFsDirectory {
    pub id: String,
    pub name: String,
    pub path: String,
    pub parent: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}
