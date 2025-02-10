#[sai_macros::element("page")]
pub fn Start(style: String) -> Element {
    rsx! {
        style { { style } }
        main {
            article {
                section {
                    button { class: "search", onclick: |_| { println!("search"); }, div { class: "icon search" } }
                    button { class: "new", onclick: |_| { println!("new"); }, div { class: "icon new" } }
                    button { class: "editor",  onclick: |_| { println!("editor"); }, div { class: "icon", id: "editor" } }
                }
            }
        }
    }
}

use crate::platform::Window;
pub fn StartWindow() -> Window {
    #[cfg(feature = "desktop")]
    {
        Window::new(super::Start, || {
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            let size = LogicalSize::new(480, 330);
            Config::new().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_max_inner_size(size)
                    .with_min_inner_size(size)
                    .with_resizable(true)
                    .with_maximizable(false)
                    .with_transparent(true)
                    .with_title(NAME),
            )
        })
    }
    #[cfg(feature = "web")]
    {
        Window::new(NAME)
    }
}
