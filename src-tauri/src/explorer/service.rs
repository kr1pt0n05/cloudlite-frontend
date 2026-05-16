use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::local::models::local_fs_file::LocalFsFile;
use crate::db::service::DBService;
use crate::fs::service::FilesystemService;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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

    // ToDo: Handle errors, e.g. if a directory does not exist, give user feedback
    pub async fn write_dropped_paths(
        &self,
        app: AppHandle,
        paths: Vec<String>,
        destination_path: Option<String>,
    ) -> Result<(), String> {
        // Convert user's absolute path to relative path to tauri's app volume
        // Important: Append relative to destination_path (relative path where user dropped the directory/file)
        // Copy from path, e.g. /home/user/dropped/file.txt
        // to path, e.g. tauri_base_path/dropped/file.txt

        for path in paths {
            let root = Path::new(&path);

            if let Some(parent) = root.parent() {
                // Walk each path recursively
                for path in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
                    let entry = Path::new(path.path());
                    // Strip root folder, e.g.:
                    // Root = /home/user/Folder1, entry = /home/user/Folder1/Subfolder1/File.txt
                    // Stripped = Folder1/Subfolder1/File.txt
                    if let Ok(stripped) = entry.strip_prefix(parent) {
                        let event_path =
                            PathBuf::from(destination_path.as_deref().unwrap_or_default())
                                .join(stripped)
                                .to_string_lossy()
                                .to_string();

                        if entry.is_dir() {
                            let directory = PathBuf::from(&self.fs.get_base_path())
                                .join(destination_path.as_deref().unwrap_or_default())
                                .join(stripped);
                            self.fs.create_directory(directory)
                                .expect("Failed to create directory");



                            // Notify frontend
                            app.emit(
                                "filesystem-entry-created",
                                crate::fs::service::FilesystemEntryCreated {
                                    path: event_path.clone(),
                                    is_directory: true,
                                },
                            )
                                .expect("Failed to emit filesystem entry created");
                        }
                        // ToDo: Error handling
                        if entry.is_file() {
                            let source = PathBuf::from(path.path());
                            let destination = PathBuf::from(&self.fs.get_base_path())
                                .join(destination_path.as_deref().unwrap_or_default())
                                .join(stripped);
                            fs::copy(source, destination)
                                .map_err(|e| format!("Failed to copy file: {}", e))?;

                            // Notify frontend
                            app.emit(
                                "filesystem-entry-created",
                                crate::fs::service::FilesystemEntryCreated {
                                    path: event_path,
                                    is_directory: false,
                                },
                            )
                                .expect("Failed to emit filesystem entry created");
                        }
                    }
                }
            }
        }

        // ToDo: Implement streaming?

        // Write batches to filesystem
        // ToDo: Handle conflicts (Prompt user: Cancel or overwrite)

        // Write batches to sqlite

        // Notify frontend
        Ok(())
    }

}
