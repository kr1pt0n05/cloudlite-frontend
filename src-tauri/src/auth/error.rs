#[derive(Debug)]
pub enum AuthError {
    MissingPkceVerifier,
    TokenRequestFailed(String),
    MutexPoisoned
}