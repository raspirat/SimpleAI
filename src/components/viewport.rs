#[sai_macros::element("component")]
pub fn Viewport() -> Element {
    use super::{Node, NodeProps};
    use dioxus::html::geometry::*;

    let mut nodes = use_signal(Vec::<NodeProps>::new);

    let drop = move |e: Event<DragData>| {
        e.prevent_default();
        for _ in 0..1000 {
            nodes.push(
                NodeProps::builder()
                    .name(crate::global::DRAG_CONTEXT())
                    .build(),
            );
        }
        // nodes.push(
        //     NodeProps::builder()
        //         .name(crate::global::DRAG_CONTEXT())
        //         .build(),
        // );
    };

    let dragover = move |e: DragEvent| {
        e.prevent_default();
    };

    let mut cursor_start_coords = use_signal(PagePoint::origin);
    let mut element_start_coords = use_signal(PagePoint::origin);

    let mut position = use_signal(|| PagePoint::origin());
    let mut scale = use_signal(|| 1f32);
    let mut pressed = use_signal(|| false);
    let mut pressed_node = use_signal(|| Option::<NodeProps>::None);
    let mut cursor = use_signal(String::new);

    let get_coords = move |e: &MouseEvent| -> PagePoint {
        let d = e.page_coordinates() - cursor_start_coords();
        element_start_coords() + d
    };

    let mousedown = move |e: MouseEvent| {
        for mut node in nodes.iter_mut() {
            if (node.pressed)() {
                node.cursor.set("grab".into());
                cursor_start_coords.set(e.page_coordinates());
                element_start_coords.set((node.position)());
                pressed_node.set(Some(node.clone()));
                return;
            }
        }
        if let Some(button) = e.trigger_button() {
            if button.into_web_code() == 1 {
                cursor_start_coords.set(e.page_coordinates());
                element_start_coords.set(position());
                pressed.set(true);
            }
        }
    };

    let mousemove = move |e: MouseEvent| {
        if let Some(mut node) = pressed_node() {
            node.cursor.set("grabbing".into());
            node.position.set(get_coords(&e));
        } else if pressed() {
            cursor.set("move".into());
            position.set(get_coords(&e));
        }
    };

    let mouseup = move |e: MouseEvent| {
        if let Some(mut node) = pressed_node() {
            node.cursor.set("grab".into());
            node.position.set(get_coords(&e));
            node.pressed.set(false);
            pressed_node.set(None);
        } else if pressed() {
            cursor.set("unset".into());
            position.set(get_coords(&e));
            pressed.set(false);
        }
    };

    let scroll = move |e: ScrollEvent| {
        e.prevent_default();
        scale -= 0.01;
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
            onscroll: scroll,
            main {
                position: "absolute",
                overflow: "visible",
                top: 0,
                left: 0,
                transform: "translate({position().x}px, {position().y}px) scaleZ({scale()})",
                user_select: "none",
                for node in nodes.iter() {
                   { Node( node.clone() ) }
                }
            }
        }
    }
}
