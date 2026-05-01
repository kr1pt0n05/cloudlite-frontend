#[derive(Debug)]
pub enum AuthError {
    MissingPkceVerifier,
    MissingAuthURL,
    TokenRequestFailed(String),
    MutexPoisoned
}