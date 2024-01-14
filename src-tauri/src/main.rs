// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use tauri::{
    api::shell, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

#[tauri::command]
fn create_note(folder_path: String, note_title: String, note_content: String) {
    let file_name = format!("{}/{}.md", folder_path, note_title.replace(" ", "_"));
    fs::write(file_name.clone(), note_content).expect("Unable to write file");
    println!("Note created: {}", file_name.clone().to_string());
}

fn main() {
    let new_note = CustomMenuItem::new("new_note", "New Note");
    let settings = CustomMenuItem::new("settings", "Settings");
    let about = CustomMenuItem::new("about", "About");
    let quit = CustomMenuItem::new("quit", "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(new_note)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.emit("open_main", ()).unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "new_note" => {
                    let window = app.get_window("main").unwrap();
                    window.emit("new_note", ()).unwrap();
                }
                "settings" => {
                    let window = app.get_window("main").unwrap();
                    window.emit("open_settings", ()).unwrap();
                }
                "about" => {
                    // open browser to github page
                    let _ = shell::open(
                        &app.shell_scope(),
                        "https://github.com/ltsbt/obsidian-quicknote",
                        None,
                    );
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![create_note])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
