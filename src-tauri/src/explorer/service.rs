use crate::db::local::models::local_fs_directory::LocalFsDirectory;
use crate::db::local::models::local_fs_file::LocalFsFile;
use crate::db::service::DBService;
use crate::fs::service::FilesystemService;
use crate::utils::time::system_time_to_datetime;
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;
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

    pub async fn get_directory_entries(
        &self,
        directory_id: Option<String>,
    ) -> Result<DirectoryEntries, sqlx::Error> {
        let directories = self.db.get_subdirectories(&directory_id).await?;
        let files = self.db.get_files_of_directory(&directory_id).await?;
        Ok(DirectoryEntries { directories, files })
    }

    pub async fn rename_file(
        &self,
        file_id: String,
        filename: String,
    ) -> Result<(), String> {
        let mut file = self.db.get_file_by_id(&file_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "File not found".to_string())?;

        println!("Renaming {} to {}", file_id, filename);
        println!("File info: {:?}", file);

        let relative_file_path = self.db.get_file_path_by_file_id(&file_id)
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or_default();

        println!("Relative file path: {:?}", relative_file_path);

        let absolute_file_path = self.fs.to_absolute_path(&relative_file_path)
        .map_err(|e| e.to_string())?;
        println!("Absolute file path: {:?}", absolute_file_path);

        let renamed_absolute_file_path = absolute_file_path.clone().join(&filename);
        println!("Renamed absolute file path: {:?}", renamed_absolute_file_path);

        self.fs.rename(&absolute_file_path, &renamed_absolute_file_path)
            .map_err(|e| e.to_string())?;

        let metadata = self.fs.metadata(&renamed_absolute_file_path)
            .map_err(|e| e.to_string())?;

        // Overwrite file
        file.name = filename;
        file.updated_at = Option::from(metadata.modified()
            .map(system_time_to_datetime)
            .unwrap_or_else(|_| Utc::now().to_rfc3339()));

        self.db.patch_local_file(file)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    // ToDo: Handle errors, e.g. if a directory does not exist, give user feedback
    // ToDo: Refactor this function
    pub async fn write_dropped_paths(
        &self,
        app: AppHandle,
        paths: Vec<String>,
        destination_path: Option<String>,
    ) -> Result<(), String> {
        // Map to store directory paths and their generated IDs for quick lookup when processing files
        let mut directory_ids: HashMap<String, String> = HashMap::new();

        if let Some(destination) = destination_path.as_ref() {
            let destination_directory_id = self
                .db
                .get_directory_by_path(destination)
                .await
                .map_err(|e| format!("Failed to query destination directory: {}", e))?
                .ok_or_else(|| "Destination directory not found in database.".to_string())?;

            directory_ids.insert(destination.clone(), destination_directory_id);
        }

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
                            let metadata = entry.metadata().unwrap(); // ToDo: Handle error

                            // Write to filesystem
                            let path = PathBuf::from(&self.fs.get_base_path())
                                .join(destination_path.as_deref().unwrap_or_default()) // ToDo: Handle error
                                .join(stripped);
                            self.fs
                                .create_directory(&path)
                                .expect("Failed to create directory");

                            // Generate a stable ID for this directory and store a path
                            // The map allows child entries to quickly resolve their parent directory ID later
                            // If no parent ID is found, the directory is treated as a root-level entry
                            let id = Uuid::new_v4().to_string();

                            let relative_path = self.fs.to_relative_path(path.as_path())?;
                            let relative_path_str = relative_path
                                .to_str()
                                .ok_or_else(|| "Invalid UTF-8 in path".to_string())?
                                .to_string();

                            let parent_id = path
                                .parent()
                                .and_then(|parent_path| self.fs.to_relative_path(parent_path).ok())
                                .and_then(|relative_parent_path| {
                                    relative_parent_path.to_str().map(|s| s.to_string())
                                })
                                .and_then(|relative_parent_path_str| {
                                    directory_ids.get(&relative_parent_path_str).cloned()
                                });

                            directory_ids.insert(relative_path_str.clone(), id.clone());

                            // Save to database
                            // ToDo: Batch
                            self.db
                                .save_local_directory(LocalFsDirectory {
                                    id: id.clone(),
                                    name: entry.file_name().unwrap().to_string_lossy().to_string(), // ToDo: Handle error
                                    path: relative_path_str,
                                    parent: parent_id,
                                    created_at: metadata
                                        .created()
                                        .map(system_time_to_datetime)
                                        .unwrap_or_else(|_| Utc::now().to_rfc3339()),
                                    updated_at: None,
                                })
                                .await
                                .map_err(|e| {
                                    format!("Failed to save directory to database: {}", e)
                                })?;

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
                            let metadata = entry.metadata().unwrap(); // ToDo: Handle error
                            let source = PathBuf::from(path.path());

                            // Write to filesystem
                            let destination = PathBuf::from(&self.fs.get_base_path())
                                .join(destination_path.as_deref().unwrap_or_default())
                                .join(stripped);
                            fs::copy(&source, &destination)
                                .map_err(|e| format!("Failed to copy file: {}", e))?;

                            let directory_id = destination
                                .parent()
                                .and_then(|parent_path| self.fs.to_relative_path(parent_path).ok())
                                .and_then(|relative_parent_path| {
                                    relative_parent_path.to_str().map(|s| s.to_string())
                                })
                                .and_then(|relative_parent_path_str| {
                                    directory_ids.get(&relative_parent_path_str).cloned()
                                });

                            // Save to database
                            // ToDo: Batch
                            self.db
                                .save_local_file(LocalFsFile {
                                    id: Uuid::new_v4().to_string(),
                                    name: entry.file_name().unwrap().to_string_lossy().to_string(), // ToDo: Handle error
                                    directory: directory_id, // ToDo: Query directory beforehand and insert id here
                                    checksum: None,          // ToDo: Calculate checksum
                                    size: metadata.len() as i64,
                                    mime_type: mime_guess::from_path(&source)
                                        .first_or_octet_stream()
                                        .to_string(), // ToDo: Handle error and get real mime type, might use infer?
                                    created_at: metadata
                                        .created()
                                        .map(system_time_to_datetime)
                                        .unwrap_or_else(|_| Utc::now().to_rfc3339()), // ToDo: Handle error
                                    updated_at: None,
                                })
                                .await
                                .map_err(|e| format!("Failed to save file to database: {}", e))?;

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

        Ok(())
    }
}
