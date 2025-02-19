use dioxus::html::geometry::{euclid::Vector2D, *};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InternNode {
    #[props(into)]
    pub name: String,
    #[props(default = Signal::default())]
    pub pressed: Signal<bool>,
    #[props(default = Signal::default())]
    pub position: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub cursor: Signal<String>,
}

#[sai_macros::element("component")]
pub fn Node(style: String, intern: InternNode) -> Element {
    let mousedown = move |_| {
        intern.pressed.set(true);
    };
    rsx! {
        style { { style } }
        body {
            class: "Node",
            position: "absolute",
            top: 0,
            left: 0,
            transform: "translate({(intern.position)().x}px, {(intern.position)().y}px) scale(100%)",
            z_index: 1,
            header {
                cursor: "{intern.cursor}",
                user_select: "none",
                onmousedown: mousedown,
                onmouseover: move |_| { intern.cursor.set("grab".into()) },
                h1 { { intern.name } }
            }
            main {
            }
            footer {

            }
        }
    }
}
