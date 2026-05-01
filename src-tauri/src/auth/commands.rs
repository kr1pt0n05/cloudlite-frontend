
use crate::AppState;
use crate::auth::error::AuthError;

#[tauri::command]
pub async fn start_login(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.auth.start_login(app).await.map_err(|e| match e {
        AuthError::MissingPkceVerifier => "PKCE verifier is missing".to_string(),
        AuthError::TokenRequestFailed(msg) => format!("Token request failed: {}", msg),
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
    })?;
    Ok(())
}
