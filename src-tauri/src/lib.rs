mod auth;

use crate::auth::service::{AuthConfig, AuthService};

pub struct AppState {
    pub auth: AuthService,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            auth: AuthService::new(AuthConfig {
                client_id: "frontend-client".to_string(),
                auth_url: "http://localhost:8080/realms/development/protocol/openid-connect/auth".to_string(),
                token_url: "http://localhost:8080/realms/development/protocol/openid-connect/token".to_string(),
                redirect_url: "http://localhost:4200".to_string(),
            }).expect("auth service should initialize"),
        })
        .invoke_handler(tauri::generate_handler![auth::commands::start_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
