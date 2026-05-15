use std::sync::Arc;
use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::local::models::local_fs_file::LocalFsFile;
use crate::db::service::DBService;
use crate::fs::service::FilesystemService;

pub struct DirectoryEntries {
    pub directories: Vec<LocalFsDirectory>,
    pub files: Vec<LocalFsFile>,
}

pub struct ExplorerService {
    db: Arc<DBService>,
    fs: Arc<FilesystemService>,
}

impl ExplorerService {
    pub fn new(db: Arc<DBService>, fs: Arc<FilesystemService>) -> Self {
        Self { db, fs }
    }

    pub async fn get_directory_entries(&self, directory_id: Option<String>) -> Result<DirectoryEntries, sqlx::Error> {
        let directories = self.db.get_subdirectories(&directory_id).await?;
        let files = self.db.get_files_of_directory(&directory_id).await?;
        Ok(DirectoryEntries { directories, files })
    }

}