use tauri::{App, AppHandle, generate_handler, Manager};
use libbacksne::info::{Info, Infos};
use crate::page::Page;
use crate::search::Search;

static SEARCH: Search = Search::from(); // todo: change path

fn get_app() -> App
{
	tauri::Builder::default()
		.invoke_handler(generate_handler![start_page, search_page])
		.build(tauri::generate_context!())
		.expect("error while running tauri application")
}

pub fn run()
{
	let app: App = get_app();

	start_page(app.app_handle());

	app.run(|x, event| {});
}

#[tauri::command]
fn start_page(app_handle: AppHandle)
{
	Page::new(&app_handle, "start_page", "/pages/start_page/html/index.html", 700, 500);
}

#[tauri::command]
fn search_page(app_handle: AppHandle)
{
	Page::new(&app_handle, "search_page", "/pages/search_page/html/index.html", 700, 1000);
}

#[tauri::command]
fn search(query: &str) -> Infos
{
	return .search(query);
}
