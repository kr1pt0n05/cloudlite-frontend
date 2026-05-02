
use crate::AppState;
use crate::auth::error::AuthError;

#[tauri::command]
pub async fn begin_login(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let auth_url = state.auth.begin_login().await.map_err(|e| match e {
        AuthError::MissingPkceVerifier => "PKCE verifier is missing".to_string(),
        AuthError::TokenRequestFailed(msg) => format!("Token request failed: {}", msg),
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
        AuthError::MissingAuthURL => "Auth URL is missing".to_string(),
        _ => {"Unkown error.".to_string()}
    })?;
    Ok(auth_url)
}

#[tauri::command]
pub async fn confirm_login(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.auth.confirm_login(app).await.map_err(|e| match e {
        AuthError::TokenRequestFailed(msg) => msg,
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
        AuthError::MissingAuthURL => "Auth URL is missing".to_string(),
        _ => "An unexpected error occurred".to_string(),
    })?;
    Ok(())
}