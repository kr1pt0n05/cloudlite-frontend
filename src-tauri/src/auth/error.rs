#[derive(Debug)]
pub enum AuthError {
    MissingPkceVerifier,
    MissingAuthURL,
    TokenRequestFailed(String),
    MutexPoisoned,
    ServerStartFailed,
    UnexpectedState,
    CodeExchangeFailed,
    MissingCsrfToken,
    InvalidCSRFToken,
    APIRequestError(String),
}