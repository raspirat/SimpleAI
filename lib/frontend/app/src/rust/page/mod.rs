use tauri::{AppHandle, LogicalSize, Manager, Window};

/// page struct for easier use of tauri Windows
pub struct Page
{
	window: Window
}

impl Page
{
	pub fn new(app: &AppHandle, name: &str, path: &str, width: i16, height: i16) -> Option<Self>
	{
		let page: Option<Window> = app.get_window("search_page");
		if (page.is_some())
		{
			return None;
		}
		let window = tauri::WindowBuilder::new(
			app,
			name,
			tauri::WindowUrl::App(path.into())
		).build().expect("failed to build window");
		window.set_decorations(false).unwrap();
		window.set_size(LogicalSize::new(width, height)).unwrap();
		window.open_devtools();
		return Some(Self { window });
	}
}
