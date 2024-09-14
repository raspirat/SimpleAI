use serde::Serialize;
use tauri::{AppHandle, Event, EventHandler, LogicalSize, Manager, Window};

/// page struct for easier use of tauri Windows
#[derive(Clone)]
pub struct Page
{
	window: Window
}

impl Page
{
	pub fn new(app: AppHandle, name: &str, path: &str, width: i16, height: i16) -> Self
	{
		let page: Option<Window> = app.get_window(name);
		if page.is_some()
		{
			let uw_page = page.unwrap();
			uw_page.set_focus().unwrap();
			return Self { window: uw_page};
		}
		let window = tauri::WindowBuilder::new(
			&app,
			name,
			tauri::WindowUrl::App(path.into())
		).build().expect("failed to build window");
		window.set_decorations(false).unwrap();
		window.set_size(LogicalSize::new(width, height)).unwrap();
		// window.open_devtools();
		return Self { window };
	}

	pub fn emit<S: Serialize + Clone>(&self, event: &str, payload: S)
	{
		self.window.emit(event, payload).unwrap()
	}

	pub fn listen<F>(&self, event: impl Into<String>, handler: F) -> EventHandler
	where F: Fn(Event) + Send + 'static
	{
		self.window.listen(event, handler)
	}

}
