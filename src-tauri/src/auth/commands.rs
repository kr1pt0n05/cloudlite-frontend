use std::sync::Arc;
use crate::auth::error::AuthError;
use crate::auth::service::AuthService;

#[tauri::command]
pub async fn begin_login(state: tauri::State<'_, Arc<AuthService>>) -> Result<String, String> {
    let auth_url = state.begin_login().await.map_err(|e| match e {
        AuthError::MissingPkceVerifier => "PKCE verifier is missing".to_string(),
        AuthError::TokenRequestFailed(msg) => format!("Token request failed: {}", msg),
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
        AuthError::MissingAuthURL => "Auth URL is missing".to_string(),
        _ => {"Unkown error.".to_string()}
    })?;
    Ok(auth_url)
}

#[tauri::command]
pub async fn confirm_login(app: tauri::AppHandle, state: tauri::State<'_, Arc<AuthService>>) -> Result<(), String> {
    state.confirm_login(app).await.map_err(|e| match e {
        AuthError::TokenRequestFailed(msg) => msg,
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
        AuthError::MissingAuthURL => "Auth URL is missing".to_string(),
        _ => "An unexpected error occurred".to_string(),
    })?;
    Ok(())
}


#[tauri::command]
pub fn is_authenticated(state: tauri::State<'_, Arc<AuthService>>) -> Result<bool, String> {
    state.is_authenticated().map_err(|e| match e {
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
        _ => "An unexpected error occurred".to_string(),
    })
}