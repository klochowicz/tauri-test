use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

const ACTION_QUIT: &str = "quit";
const ACTION_CLOSE: &str = "close";
const ACTION_PREFERENCES: &str = "preferences";

pub fn make_menu() -> Menu {
    // Add main app menu
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let file_submenu = {
        let quit = CustomMenuItem::new(ACTION_QUIT.to_string(), "Quit");
        let close = CustomMenuItem::new(ACTION_CLOSE.to_string(), "Close");
        Submenu::new("File", Menu::new().add_item(quit).add_item(close))
    };
    let options_submenu = {
        let preferences = CustomMenuItem::new(ACTION_PREFERENCES, "Prefenences");
        Submenu::new("Options", Menu::new().add_item(preferences))
    };
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(file_submenu)
        .add_submenu(options_submenu)
}

pub fn handle_menu_events(event: WindowMenuEvent) {
    match event.menu_item_id() {
        ACTION_QUIT => {
            std::process::exit(0);
        }
        ACTION_CLOSE => {
            event.window().close().unwrap();
        }
        ACTION_PREFERENCES => {
            // TODO: Add showing preferences window
        }
        _ => {}
    }
}
