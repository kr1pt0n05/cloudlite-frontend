use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;

pub struct FilesystemService {
    base_path: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FilesystemEntryCreated {
    path: String,
    is_directory: bool,
}

impl FilesystemService {
    //ToDo: Handle base path
    pub fn new() -> Self {
        Self {
            base_path: "/home/sander/Dokumente/CloudLite".to_owned(),
        }
    }

    pub fn create_directory<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        fs::create_dir(path.as_ref()).map_err(|e| format!("Failed to create directory: {}", e))?;
        Ok(())
    }

    pub fn create_directory_all<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        fs::create_dir_all(path.as_ref())
            .map_err(|e| format!("Failed to create directories: {}", e))?;
        Ok(())
    }

    /// Takes "/Users/name/Desktop/MyFolder"
    /// and returns "[AppDir]/MyFolder"
    fn localize_user_path(
        &self,
        user_absolute_path: &str,
        destination_path: Option<&str>,
    ) -> PathBuf {
        let user_path = Path::new(user_absolute_path);
        let folder_name = user_path.file_name().unwrap_or_default(); // 1. Get the end of the path safely
        let mut root = PathBuf::from(&self.base_path);

        if let Some(destination) = destination_path {
            &root.push(Path::new(destination));
        }
        root.join(folder_name)
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
                            let directory = PathBuf::from(&self.base_path)
                                .join(destination_path.as_deref().unwrap_or_default())
                                .join(stripped);
                            self.create_directory(directory)
                                .expect("Failed to create directory");

                            // Notify frontend
                            app.emit(
                                "filesystem-entry-created",
                                FilesystemEntryCreated {
                                    path: event_path.clone(),
                                    is_directory: true,
                                },
                            )
                            .expect("Failed to emit filesystem entry created");
                        }
                        // ToDo: Error handling
                        if entry.is_file() {
                            let source = PathBuf::from(path.path());
                            let destination = PathBuf::from(&self.base_path)
                                .join(destination_path.as_deref().unwrap_or_default())
                                .join(stripped);
                            fs::copy(source, destination)
                                .map_err(|e| format!("Failed to copy file: {}", e))?;

                            // Notify frontend
                            app.emit(
                                "filesystem-entry-created",
                                FilesystemEntryCreated {
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
