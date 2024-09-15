use serde::Serialize;
use tauri::{AppHandle, Event, EventHandler, LogicalSize, Manager, Window};

/// page struct for easier use of tauri Windows
#[derive(Clone)]
pub struct Page {
    window: Window,
}

impl Page {
    pub fn new(
        app: AppHandle,
        name: String,
        path: String,
        width: i16,
        height: i16,
        fullscreen: bool,
    ) -> Self {
        let name_clone = name.clone();
        let app_clone = app.clone();
        let page: Option<Window> = app.get_window(&name);
        if page.is_some() {
            let uw_page = page.unwrap();
            uw_page.set_focus().unwrap();
            return Self { window: uw_page };
        }
        let t_handle = std::thread::spawn(move || {
            let window = tauri::WindowBuilder::new(&app, &name, tauri::WindowUrl::App(path.into()))
                .decorations(false)
                .fullscreen(fullscreen)
                .build()
                .expect("failed to build window");
            window.set_size(LogicalSize::new(width, height)).unwrap();
        });
        t_handle.join().unwrap();
        // window.open_devtools();
        Self {
            window: app_clone
                .get_window(&name_clone)
                .expect("Window couldn't be built!"),
        }
    }

    pub fn emit<S: Serialize + Clone>(&self, event: &str, payload: S) {
        self.window.emit(event, payload).unwrap()
    }

    pub fn listen<F>(&self, event: impl Into<String>, handler: F) -> EventHandler
    where
        F: Fn(Event) + Send + 'static,
    {
        self.window.listen(event, handler)
    }
}
