use tauri::{App, AppHandle, generate_handler, Manager};
use libbacksne::{
	info::{Info, Infos},
	stdpaths::{node_path},
	errors::{*},
	search::{*}
};

use crate::{
	page::{Page},
};

fn get_app() -> App
{
	tauri::Builder::default()
		.invoke_handler(
			generate_handler![
				start_page,
				search_page,
				settings_page,
				new_node_page,
				editor_page,
				search
			]
		)
		.build(tauri::generate_context!())
		.expect("error while running tauri application")
}

pub fn run()
{
	let app: App = get_app();

	start_page(app.app_handle());

	app.run(|x, event| {});
}


fn i_start_page(app_handle: AppHandle) -> Page
{
	Page::new(app_handle, "start_page", "/pages/start_page/html/index.html", 700, 500)
}

#[tauri::command]
fn start_page(app_handle: AppHandle)
{
	i_start_page(app_handle);
}

fn i_new_node_page(app_handle: AppHandle) -> Page
{
	Page::new(app_handle, "new_node_page", "/pages/new_node_page/html/index.html", 700, 1000)
}
#[tauri::command]
fn settings_page(app_handle: AppHandle)
{
	i_settings_page(app_handle);
}

fn i_settings_page(app_handle: AppHandle) -> Page
{
	Page::new(app_handle, "settings_page", "/pages/settings_page/html/index.html", 600, 600)
}

#[tauri::command]
fn new_node_page(app_handle: AppHandle)
{
	i_new_node_page(app_handle);
}

fn i_search_page(app_handle: AppHandle) -> Page
{
	Page::new(app_handle, "search_page", "/pages/search_page/html/index.html", 700, 1000)
}

#[tauri::command]
fn search_page(app_handle: AppHandle)
{
	i_search_page(app_handle);
}

fn error_page(app_handle: AppHandle, error: Error) -> Page
{
	println!("errorpage");
	let p: Page = Page::new(

		app_handle,
		"error_page",
		"/pages/error_page/html/index.html",
		300,
		300
	);
	let pc = p.clone();
	let ev = pc.listen("get_error_message", move |_| {
		p.emit("error_message", error.clone());
	});
	pc
}

#[tauri::command]
fn editor_page(app_handle: AppHandle)
{
	Page::new(app_handle, "editor_page", "/pages/editor_page/html/index.html", 1920, 1080);
}

#[tauri::command]
fn search(app_handle: AppHandle, query: &str) -> Vec<SearchResult>
{
	return match Search::new()
	{
		Ok(s) => {s.search(query).unwrap_or_default().into_vec()}
		Err(e) => {error_page(app_handle, e); SearchResults::default().into_vec()}
	}
}
