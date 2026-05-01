// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;

#[tokio::main]
async fn main() {
    cloudlite_frontend_lib::run()
}
