mod auth;
mod api;
mod db;
mod sync;
mod fs;
mod explorer;
mod utils;

use std::sync::Arc;
use crate::auth::service::{AuthConfig, AuthService};
use crate::api::service::{ApiService};
use crate::auth::commands::{begin_login, confirm_login, is_authenticated};
use crate::sync::commands::run_sync;
use crate::db::service::DBService;
use crate::fs::service::FilesystemService;
use crate::sync::service::SyncService;
use crate::explorer::commands::{get_directory_entries, receive_dropped_paths};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let auth_service = Arc::new(AuthService::new(
        AuthConfig {
        client_id: "frontend-client".to_string(),
        auth_url: "http://localhost:8080/realms/development/protocol/openid-connect/auth".to_string(),
        token_url: "http://localhost:8080/realms/development/protocol/openid-connect/token".to_string(),
        redirect_url: "http://localhost:4200".to_string(),
    }).expect("auth sync should initialize"));

    let api_service = Arc::new(ApiService::new(
        Arc::clone(&auth_service),
        "http://localhost:8000/api".to_string()
    ));

    let db_service = Arc::new(DBService::new().await);

    let fs_service = Arc::new(FilesystemService::new());

    let explorer_service = Arc::new(explorer::service::ExplorerService::new(
        Arc::clone(&db_service),
        Arc::clone(&fs_service),
    ));

    let sync_service = Arc::new(SyncService::new(
        Arc::clone(&api_service),
        Arc::clone(&db_service),
        Arc::clone(&fs_service)
    ));


    // Development only
    // ToDo: Remove for production
    db_service.drop_all_tables().await;
    fs_service.remove_base_directory();

    fs_service.create_base_directory();

    db_service.create_changelogs_if_not_exists().await.expect("Failed to create changelogs table");
    db_service.create_local_fs_directory_if_not_exists().await.expect("Failed to create local_fs_directories table");
    db_service.create_local_fs_file_if_not_exists().await.expect("Failed to create local_fs_files table");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(auth_service)
        .manage(api_service)
        .manage(db_service)
        .manage(fs_service)
        .manage(explorer_service)
        .manage(sync_service)
        .invoke_handler(tauri::generate_handler![
            begin_login,
            confirm_login,
            is_authenticated,
            run_sync,
            receive_dropped_paths,
            get_directory_entries
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
