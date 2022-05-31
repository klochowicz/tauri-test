#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
fn main() {
// here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
      .add_native_item(MenuItem::Copy)
      .add_item(CustomMenuItem::new("hide", "Hide"))
      .add_submenu(submenu);

  tauri::Builder::default()
      .manage(Database {})
      .menu(menu)
      .on_menu_event(|event| {
        match event.menu_item_id() {
          "quit" => {
            std::process::exit(0);
          }
          "close" => {
            event.window().close().unwrap();
          }
          _ => {}
        }
      })
      .invoke_handler(tauri::generate_handler![my_custom_command])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
