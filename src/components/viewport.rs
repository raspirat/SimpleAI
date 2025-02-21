#[sai_macros::element("component")]
pub fn Viewport() -> Element {
    use super::{InternNode, Node};
    use dioxus::html::geometry::{euclid::*, *};

    let mut nodes = use_signal(Vec::<InternNode>::new);

    let drop = move |e: Event<DragData>| {
        e.prevent_default();

        let mut node = InternNode::from(crate::global::context::DRAG_NODE.unwrap());
        node.position.set(e.page_coordinates().to_vector());
        nodes.push(node);
    };

    let dragover = move |e: DragEvent| {
        e.prevent_default();
    };

    let mut cursor_start_coords = use_signal(Vector2D::<f64, _>::zero);
    let mut element_start_coords = use_signal(Vector2D::<f64, _>::zero);

    let mut position = use_signal(Vector2D::<f64, _>::zero);
    let mut scale = use_signal(|| 1f64);
    let mut pressed = use_signal(|| false);
    let mut pressed_node = use_signal(|| Option::<InternNode>::None);
    let mut cursor = use_signal(String::new);

    let get_coords = move |e: &MouseEvent| -> Vector2D<f64, _> {
        let d = e.page_coordinates().to_vector() - cursor_start_coords();
        element_start_coords() + d
    };

    let get_node_coords = move |e: &MouseEvent| -> Vector2D<f64, _> {
        {
            get_coords(e) / scale()
        }
    };

    let mousedown = move |e: MouseEvent| {
        for mut node in nodes.iter_mut() {
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
    };

    let mousemove = move |e: MouseEvent| {
        if let Some(mut node) = pressed_node() {
            node.cursor.set("grabbing".into());
            node.position.set(get_node_coords(&e));
        } else if pressed() {
            cursor.set("move".into());
            position.set(get_coords(&e));
        }
    };

    let mouseup = move |e: MouseEvent| {
        if let Some(mut node) = pressed_node() {
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

    let rendered_nodes = nodes
        .iter()
        .map(|intern| rsx! { Node { intern: intern.clone() } });

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
