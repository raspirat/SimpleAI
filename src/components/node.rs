use crate::components::*;
use crate::utils;
use dioxus::html::geometry::{euclid::Vector2D, *};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InternNode {
    pub node: utils::StrongNode,
    #[props(default = Signal::default())]
    pub params: Signal<Vec<InternParam>>,
    #[props(default = Signal::default())]
    pub pressed: Signal<bool>,
    #[props(default = Signal::default())]
    pub position: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub cursor: Signal<String>,
}

impl From<utils::StrongNode> for InternNode {
    fn from(node: utils::StrongNode) -> Self {
        let b = Self::builder();
        if let Ok(data) = node.context.lock() {
            b.params(Signal::new(
                data.params
                    .iter()
                    .map(|param| InternParam::from(*param))
                    .collect(),
            ));
        }
        b.node(node).build()
    }
}

#[sai_macros::element("component")]
pub fn Node(style: String, intern: InternNode) -> Element {
    let mousedown = move |_| {
        intern.pressed.set(true);
    };

    let mounted = move |e: MountedEvent| {
        //     intern.node.params.iter().for_each(|param| {
        //         intern.params.push(InternParam::from(param.clone()));
        //     });
    };

    let rendered_params = intern
        .params
        .iter()
        .map(|intern| rsx! { Param { intern: intern.clone() } });

    rsx! {
        style { { style } }
        body {
            class: "Node",
            position: "absolute",
            top: 0,
            left: 0,
            transform: "translate({(intern.position)().x}px, {(intern.position)().y}px) scale(100%)",
            z_index: 1,
            onmounted: mounted,
            header {
                cursor: "{intern.cursor}",
                user_select: "none",
                onmousedown: mousedown,
                onmouseover: move |_| { intern.cursor.set("grab".into()) },
                h1 { { intern.node.name } }
            }
            main {
                display: "flex",
                flex_direction: "column",
                justify_content: "space-evenly",
                align_items: "center",
                { rendered_params }
            }
            footer {

            }
        }
    }
}
