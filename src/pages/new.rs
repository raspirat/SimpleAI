#[sai_macros::element("page")]
pub fn New(style: String, icons: Icons) -> Element {
    use crate::components::*;

    rsx! {
        style { { style } }
        main {
            form {
                LabeledBox {
                    label { for: "name", "node name" }
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                }
                LabeledBox {
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                    label { for: "name", "project name" }
                }
                input { type: "text" }
                LabeledBox {
                    label { for: "name", "project name" }
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                }
                LabeledBox {
                    input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
                    label { for: "name", "project name" }
                }
                input { type: "file" }
                input { type: "range" }

                input { type: "list", list: "options" }

                datalist {
                    id: "options",
                    option { value: "1", "1" }
                    option { value: "2", "2" }
                    option { value: "3", "3" }
                }

                button { type: "submit", "create" }
            }
        }
    }
}

use crate::platform::Window;
pub fn NewWindow() -> Window {
    #[cfg(feature = "desktop")]
    {
        Window::new(super::New, || {
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(680, 1000))
                    .with_resizable(false)
                    .with_maximizable(false)
                    .with_transparent(true),
            )
        })
    }
    #[cfg(feature = "web")]
    {
        Window::new(NAME)
    }
}
