mod auth;
mod api;
mod db;

use std::sync::Arc;
use crate::auth::service::{AuthConfig, AuthService};
use crate::api::service::{ApiService};
use crate::api::commands::{get_change_logs};
use crate::auth::commands::{begin_login, confirm_login, is_authenticated};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let auth_service = Arc::new(AuthService::new(
        AuthConfig {
        client_id: "frontend-client".to_string(),
        auth_url: "http://localhost:8080/realms/development/protocol/openid-connect/auth".to_string(),
        token_url: "http://localhost:8080/realms/development/protocol/openid-connect/token".to_string(),
        redirect_url: "http://localhost:4200".to_string(),
    }).expect("auth service should initialize"));

    let api_service = Arc::new(ApiService::new(
        Arc::clone(&auth_service),
        "http://localhost:8000/api".to_string()
    ));


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(auth_service)
        .manage(api_service)
        .invoke_handler(tauri::generate_handler![
            begin_login,
            confirm_login,
            is_authenticated,
            get_change_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
