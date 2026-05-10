#[derive(Debug)]
pub enum FilesystemError {
    IoError(String),
    MutexPoisoned,
    InvalidPath(String),
    UnknownError(String),
}