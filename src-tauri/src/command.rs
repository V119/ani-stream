use crate::spider::model::MainPage;
use crate::spider::spider::{self, get_page};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn main_page() -> Result<MainPage, String> {
    get_page(None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn next_page() -> Result<MainPage, String> {
    spider::next_page().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pre_page() -> Result<MainPage, String> {
    spider::pre_page().await.map_err(|e| e.to_string())
}
