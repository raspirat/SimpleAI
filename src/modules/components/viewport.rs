use std::ops::DerefMut;

use dioxus::html::geometry::{euclid::*, *};
use sai_backend::utils::prelude::*;

#[derive(Clone)]
pub struct ViewportNodeContainer {
    pub backend_node_container: NodeContainer,
    pub frontend_node_container: Signal<Vec<super::InternNode>>,
}
impl ViewportNodeContainer {
    pub fn new() -> Self {
        Self {
            backend_node_container: NodeContainer::new(),
            frontend_node_container: use_signal(Vec::<super::InternNode>::new),
        }
    }
    pub fn push_context(&mut self, node_ctx: StrongNode) {
        self.backend_node_container.push_context(node_ctx.clone());
        self.frontend_node_container
            .push(super::InternNode::from(node_ctx));
    }
}

#[sai_macros::element("component")]
pub fn Viewport() -> Element {
    // ------------------------------ VARIABLES ------------------------------ //
    let node_context = StrongContext::from(ViewportNodeContainer::new());

    let nc_rendered_nodes = node_context.clone();
    let rendered_nodes = {
        spawn(async move {
            let nodes = nc_rendered_nodes.context.lock().await;
            nodes
                .frontend_node_container
                .iter()
                .map(|intern| rsx! { super::Node { intern: intern.clone() } });
        });
    };

    let mut cursor_start_coords = use_signal(Vector2D::<f64, _>::zero);
    let mut element_start_coords = use_signal(Vector2D::<f64, _>::zero);

    let mut position = use_signal(Vector2D::<f64, _>::zero);
    let mut scale = use_signal(|| 1f64);

    let mut pressed = use_signal(|| false);
    let mut pressed_node = use_signal(|| Option::<super::InternNode>::None);
    let mut pressed_connection = use_signal(|| Option::<super::InternConnection>::None);

    let mut cursor = use_signal(String::new);

    let get_diff = move |e: &MouseEvent| -> Vector2D<f64, _> {
        e.page_coordinates().to_vector() - cursor_start_coords()
    };

    let get_coords =
        move |e: &MouseEvent| -> Vector2D<f64, _> { element_start_coords() + get_diff(e) };

    let get_node_coords = move |e: &MouseEvent| -> Vector2D<f64, _> {
        {
            get_coords(e) / scale()
        }
    };

    // ------------------------------ EVENTS ------------------------------ //
    let nc_drop = node_context.clone();
    let drop = move |e: DragEvent| {
        e.prevent_default();

        let node = crate::global::context::DRAG_NODE.unwrap();
        // node.position.set(e.page_coordinates().to_vector());
        let nci = nc_drop.clone();
        spawn(async move {
            let ncii = nci.clone();
            let mut nodes = ncii.context.lock().await;
            nodes.deref_mut().push_context(node.clone());
        });
    };

    let dragover = move |e: DragEvent| {
        e.prevent_default();
    };

    let nc_mousedown = node_context.clone();
    let mousedown = move |e: MouseEvent| {
        let nci = nc_mousedown.clone();
        spawn(async move {
            let mut nodes = nci.context.lock().await;
            for mut node in nodes.frontend_node_container.iter_mut() {
                for param in node.runtime_params.iter_mut() {
                    if ((param.connection)().pressed)() {
                        // let mut param_context = param.param.context.lock().await;
                        cursor_start_coords.set(e.page_coordinates().to_vector());
                        pressed_connection.set(Some((param.connection)().clone()));
                        return;
                    }
                }
                if (node.pressed)() {
                    node.cursor.set("grabbing".into());
                    cursor_start_coords.set(e.page_coordinates().to_vector());
                    element_start_coords.set((node.position)() * scale());
                    pressed_node.set(Some(node.clone()));
                    return;
                }
            }
            if let Some(button) = e.trigger_button() {
                if button.into_web_code() == 1 {
                    cursor_start_coords.set(e.page_coordinates().to_vector());
                    element_start_coords.set(position());
                    pressed.set(true);
                }
            }
        });
    };

    let mousemove = move |e: MouseEvent| {
        if let Some(mut connection) = pressed_connection() {
            connection.dimensions.set(get_diff(&e));
        } else if let Some(mut node) = pressed_node() {
            node.cursor.set("grabbing".into());
            node.position.set(get_node_coords(&e));
            // for mut param in node.runtime_params.iter_mut() {
            //     if let Some(connection) = (param.connection)() {}
            // }
        } else if pressed() {
            cursor.set("move".into());
            position.set(get_coords(&e));
        }
    };

    let mouseup = move |e: MouseEvent| {
        if let Some(mut connection) = pressed_connection() {
            connection.dimensions.set(get_diff(&e));
            connection.pressed.set(false);
            if let Some(mut c) = crate::global::context::CONNECTION() {
                c.foreign_dimensions.set((connection.dimensions)());
            }
            pressed_connection.set(None);
        } else if let Some(mut node) = pressed_node() {
            node.cursor.set("grab".into());
            node.position.set(get_node_coords(&e));
            node.pressed.set(false);
            pressed_node.set(None);
        } else if pressed() {
            cursor.set("unset".into());
            position.set(get_coords(&e));
            pressed.set(false);
        }
    };

    let wheel = move |e: WheelEvent| {
        e.prevent_default();
        let sub;
        match e.data().delta() {
            WheelDelta::Pixels(v) => sub = v.y / 1E3,
            WheelDelta::Lines(v) => sub = v.y / 1E3,
            WheelDelta::Pages(v) => sub = v.y / 1E3,
        }

        scale.set({ scale - sub }.max(0.0).min(1.0));

        dioxus::logger::tracing::debug!("sub: {sub}, scale: {scale}");
    };

    rsx! {
        body {
            class: "Viewport",
            cursor: "{cursor}",
            ondrop: drop,
            ondragover: dragover,
            onmousedown: mousedown,
            onmousemove: mousemove,
            onmouseup: mouseup,
            onwheel: wheel,
            main {
                position: "absolute",
                overflow: "visible",
                top: 0,
                left: 0,
                transform: "translate({position().x}px, {position().y}px) scale({scale()})",
                user_select: "none",
                { rendered_nodes }
            }
        }
    }
}
