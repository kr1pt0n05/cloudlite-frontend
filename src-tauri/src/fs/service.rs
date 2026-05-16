use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FilesystemEntryCreated {
    pub(crate) path: String,
    pub(crate) is_directory: bool,
}

pub struct FilesystemService {
    base_path: String,
}

impl FilesystemService {
    //ToDo: Handle base path
    pub fn new() -> Self {
        Self {
            base_path: "/home/sander/Dokumente/CloudLite".to_owned(),
        }
    }

    pub fn get_base_path(&self) -> &str {
        &self.base_path
    }

    pub fn to_relative_path(&self, path: &Path) -> Result<PathBuf, String> {
        path.strip_prefix(self.get_base_path())
            .map(|relative| relative.to_path_buf())
            .map_err(|_| format!("Path is not inside base path: {}", path.display()))
    }

    pub fn create_directory<P: AsRef<Path>>(&self,path: &P) -> Result<(), String> {
        fs::create_dir(path.as_ref()).map_err(|e| format!("Failed to create directory: {}", e))?;
        Ok(())
    }

    pub fn create_directory_all<P: AsRef<Path>>(&self, path: &P) -> Result<(), String> {
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

    pub fn create_base_directory(&self) {
        fs::create_dir_all(&self.base_path).expect("Failed to create base directory");
    }

    pub fn remove_base_directory(&self) {
        fs::remove_dir_all(&self.base_path).ok();
    }

}
