#[sai_macros::element("page")]
pub fn Editor(style: String, icons: Icons) -> Element {
    use crate::components::*;

    rsx! {
        style { { style } }
        main {
            Divider
            {
                section {
                    Viewport {}
                }
                aside {
                    nav {}
                    section {
                        Search {}
                    }
                }
            }
        }
    }
}

use crate::platform::Window;
pub fn EditorWindow() -> Window {
    #[cfg(feature = "desktop")]
    {
        Window::new(super::Editor, || {
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(1920, 1080))
                    .with_transparent(true),
            )
        })
    }
    #[cfg(feature = "web")]
    {
        Window::new(NAME)
    }
}
