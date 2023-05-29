// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;

use tauri::{AboutMetadata, CustomMenuItem, Manager, Menu, MenuItem, Submenu};
use tauri::WindowBuilder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let appmenu = Submenu::new(
        "Lain", Menu::with_items([
            MenuItem::About("Lain".to_string(), AboutMetadata::default()).into(),
            MenuItem::Separator.into(),
            CustomMenuItem::new("restart", "Restart ChatGPT").accelerator("CmdOrCtrl+Shift+R").into(),
            MenuItem::Quit.into()]));

    let submenu = Submenu::new(
        "File", Menu::new()
            .add_item(CustomMenuItem::new("reload", "Refresh the Screen").accelerator("CmdOrCtrl+R"))
            .add_item(CustomMenuItem::new("dev_tools", "Toggle Developer Tools").accelerator("CmdOrCtrl+Shift+I")));

    let menu = Menu::new()
        .add_submenu(appmenu)
        .add_submenu(submenu)
        ;

    let mut builder = tauri::Builder::default()
        .menu(menu)
        // .setup(|app| {
        //     let main_window = app.get_window("main").unwrap();
        //     main_window.menu_handle().get_item("reload").set_enabled(true);
        //     Ok(())
        // })
        // .on_menu_event(|event| {
        //     if event.menu_item_id() == "reload" {
        //         let main_window = event.window();
        //         main_window.reload().unwrap();
        //     }
        // })
        .invoke_handler(tauri::generate_handler![
        greet,
        //   window::cmd::window_reload,
        //   window::cmd::control_window,
        ])
        .on_menu_event(menu::menu_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application")

        ;
}
