#[sai_macros::element("page")]
pub fn Start(style: String, icons: Icons) -> Element {
    let search_icon = icons.get("search").expect("search icon not found").clone();
    let new_icon = icons.get("new").expect("new icon not found").clone();
    let editor_icon = icons.get("editor").expect("editor icon not found").clone();

    rsx! {
        style { { style } }
        main {
            article {
                section {
                    button { class: "search", onclick: |_| { crate::pages::search::SearchWindow().open(); }, div { class: "icon search", dangerous_inner_html: search_icon } }
                    button { class: "new", onclick: |_| { crate::pages::new::NewWindow().open(); }, div { class: "icon new", dangerous_inner_html: new_icon } }
                    button { class: "editor",  onclick: |_| { crate::pages::editor::EditorWindow().open(); }, div { class: "icon", id: "editor", dangerous_inner_html: editor_icon } }
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
            Config::default().with_menu(None).with_window(
                WindowBuilder::default()
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
