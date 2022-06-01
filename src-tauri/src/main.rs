#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Definition in main.rs

mod menu;
pub mod tray_menu;

use menu::handle_menu_events;
use tray_menu::handle_system_tray_event;

struct Database;

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}

async fn some_other_function() -> Option<String> {
    Some("response".into())
}

#[tauri::command]
async fn my_custom_command(
    window: tauri::Window,
    number: usize,
    _database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
    println!("Called this from {}", window.label());
    let result: Option<String> = some_other_function().await;
    if let Some(message) = result {
        Ok(CustomResponse {
            message,
            other_val: 42 + number,
        })
    } else {
        Err("No result".into())
    }
}

fn main() {
    let menu = menu::make_menu();
    let system_tray = tray_menu::make_system_tray();

    tauri::Builder::default()
        .manage(Database {})
        .system_tray(system_tray)
        .menu(menu)
        .on_menu_event(handle_menu_events)
        .on_system_tray_event(handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
