use crate::state::AppState;
#[tauri::command]
pub async fn login(app: tauri::AppHandle, state: tauri::State<'_, AppState>,) -> Result<(), String> {
    state.auth.login(app).await;
    Ok(())

}