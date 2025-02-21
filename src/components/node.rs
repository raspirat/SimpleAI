use crate::utils::Node as bNode;
use dioxus::html::geometry::{euclid::Vector2D, *};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InternNode {
    #[props(into)]
    pub node: bNode,
    #[props(default = Signal::default())]
    pub pressed: Signal<bool>,
    #[props(default = Signal::default())]
    pub position: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub cursor: Signal<String>,
}

impl From<bNode> for InternNode {
    fn from(node: bNode) -> Self {
        Self::builder().node(node).build()
    }
}

#[sai_macros::element("component")]
pub fn Node(style: String, intern: InternNode) -> Element {
    use crate::components::*;
    use crate::utils::NodeParam;

    let mousedown = move |_| {
        intern.pressed.set(true);
    };

    let rendered_params = intern.node.params.iter().map(|param| {
        let intern = InternParam::from(param.clone());
        rsx! { Param { intern } }
    });

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
                h1 { { intern.node.name } }
            }
            main {
                { rendered_params }
            }
            footer {

            }
        }
    }
}
