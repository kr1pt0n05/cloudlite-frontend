use std::fs;

pub struct FilesystemService {
    base_path: String,
}

impl FilesystemService {
    //ToDo: Handle base path
    pub fn new() -> Self {
        Self { base_path: "some_path".to_owned() }
    }

    pub fn create_directory(&self, path: String) -> Result<(), String> {
        fs::create_dir(path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
        Ok(())
    }

    pub fn create_directories(&self) -> Result<(), String> {
        fs::create_dir_all("some_path")
        .map_err(|e| format!("Failed to create directories: {}", e))?;
        Ok(())
    }


}


