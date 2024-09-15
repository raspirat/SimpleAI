use libbacksne::{errors::*, search::*};
use tauri::{generate_handler, App, AppHandle, Manager};

use crate::page::Page;

fn get_app() -> App {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            start_page,
            search_page,
            settings_page,
            new_node_page,
            editor_page,
            search
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Destroyed => {
                let window = event.window();
                if window.label() == "start_page" {
                    window.app_handle().exit(0);
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

pub fn run() {
    let app: App = get_app();

    start_page(app.app_handle());

    app.run(|_, _| {});
}

fn i_start_page(app_handle: AppHandle) -> Page {
    Page::new(
        app_handle,
        "start_page".to_string(),
        "pages/start_page/index.html".to_string(),
        700,
        500,
        false,
    )
}

#[tauri::command]
fn start_page(app_handle: AppHandle) {
    i_start_page(app_handle);
}

fn i_new_node_page(app_handle: AppHandle) -> Page {
    Page::new(
        app_handle,
        "new_node_page".to_string(),
        "pages/new_node_page/index.html".to_string(),
        700,
        1000,
        false,
    )
}
#[tauri::command]
fn settings_page(app_handle: AppHandle) {
    i_settings_page(app_handle);
}

fn i_settings_page(app_handle: AppHandle) -> Page {
    Page::new(
        app_handle,
        "settings_page".to_string(),
        "pages/settings_page/index.html".to_string(),
        600,
        600,
        false,
    )
}

#[tauri::command]
fn new_node_page(app_handle: AppHandle) {
    i_new_node_page(app_handle);
}

fn i_search_page(app_handle: AppHandle) -> Page {
    Page::new(
        app_handle,
        "search_page".to_string(),
        "pages/search_page/index.html".to_string(),
        700,
        1000,
        false,
    )
}

#[tauri::command]
fn search_page(app_handle: AppHandle) {
    i_search_page(app_handle);
}

fn error_page(app_handle: AppHandle, error: Error) -> Page {
    println!("errorpage");
    let p: Page = Page::new(
        app_handle,
        "error_page".to_string(),
        "pages/error_page/index.html".to_string(),
        300,
        300,
        false,
    );
    let pc = p.clone();
    pc.listen("get_error_message", move |_| {
        p.emit("error_message", error.clone());
    });
    pc
}

#[tauri::command]
fn editor_page(app_handle: AppHandle) {
    Page::new(
        app_handle,
        "editor_page".to_string(),
        "pages/editor_page/index.html".to_string(),
        1920,
        1080,
        true,
    );
}

#[tauri::command]
fn search(app_handle: AppHandle, query: &str) -> Vec<SearchResult> {
    match Search::new() {
        Ok(s) => s.search(query).unwrap_or_default().into_vec(),
        Err(e) => {
            error_page(app_handle, e);
            SearchResults::default().into_vec()
        }
    }
}
