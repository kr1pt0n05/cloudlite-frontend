/*use std::sync::Arc;
use crate::api::service::ApiService;
use crate::auth::error::AuthError;

#[tauri::command]
pub async fn get_latest_changelogs(state: tauri::State<'_, Arc<ApiService>>) -> Result<String, String> {
    state.get_latest_changelogs().await.map_err(|e| match e {
        AuthError::APIRequestError(e) => "API request error: ".to_string() + &e,
        AuthError::TokenRequestFailed(msg) => format!("Token request failed: {}", msg),
        AuthError::MutexPoisoned => "Mutex poisoned".to_string(),
        _ => "An unexpected error occurred".to_string(),
    })
}*/