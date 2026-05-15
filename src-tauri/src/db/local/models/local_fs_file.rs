use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalFsFile {
    pub id: String,
    pub name: String,
    pub directory: Option<String>,
    //pub path: String, Leave path away otherwise moves will be very expensive
    pub checksum: Option<String>, //ToDo: Make this non-Optional after implementing checksum calculation
    pub size: i64,
    pub mime_type: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}