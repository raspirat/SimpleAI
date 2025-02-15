#[sai_macros::element("page")]
pub fn Search(style: String) -> Element {
    use crate::components::*;
    rsx! {
        style { { style } }
        main {
            Search {}
        }
    }
}

use crate::platform::Window;
pub fn SearchWindow() -> Window {
    #[cfg(feature = "desktop")]
    {
        Window::new(super::Search, || {
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            Config::default()
                .with_menu(None)
                .with_window(WindowBuilder::new().with_transparent(true))
        })
    }
    #[cfg(feature = "web")]
    {
        Window::new(NAME)
    }
}
