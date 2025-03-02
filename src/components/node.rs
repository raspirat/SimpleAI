use super::*;
use dioxus::html::geometry::{euclid::Vector2D, *};
use dioxus::prelude::*;
use sai_backend::utils;

#[derive(PartialEq, Props, Clone)]
pub struct InternNode {
    pub node: utils::StrongNode,
    #[props(default = Signal::default())]
    pub runtime_params: Signal<Vec<InternRuntimeParam>>,
    #[props(default = Signal::default())]
    pub static_params: Signal<Vec<InternStaticParam>>,
    #[props(default = Signal::default())]
    pub pressed: Signal<bool>,
    #[props(default = Signal::default())]
    pub position: Signal<Vector2D<f64, PageSpace>>,
    #[props(default = Signal::default())]
    pub cursor: Signal<String>,
}

impl From<utils::StrongNode> for InternNode {
    fn from(node: utils::StrongNode) -> Self {
        let b = Self::builder().node(node.clone());
        // if let Ok(data) = node.context.lock() {
        //     return b
        //         .params(Signal::new(
        //             data.params
        //                 .iter()
        //                 .map(|param| InternParam::from(param.clone()))
        //                 .collect(),
        //         ))
        //         .build();
        // }
        b.build()
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
        .runtime_params
        .iter()
        .map(|intern| rsx! { RuntimeParam { intern: intern.clone() } });

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
                h1 { { if let Ok(node) = intern.node.context.lock() { return rsx! { { node.name.clone() } } }  } }
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
