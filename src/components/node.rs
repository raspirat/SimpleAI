use dioxus::html::geometry::*;

#[sai_macros::element("component")]
pub fn Node(
    style: String,
    #[props(into)] name: String,
    #[props(default = Signal::default())] pressed: Signal<bool>,
    #[props(default = Signal::default())] position: Signal<PagePoint>,
    #[props(default = Signal::default())] cursor: Signal<String>,
) -> Element {
    let mousedown = move |_| {
        pressed.set(true);
    };
    rsx! {
        style { { style } }
        body {
            class: "Node",
            position: "absolute",
            top: 0,
            left: 0,
            transform: "translate({position().x}px, {position().y}px)",
            z_index: 1,
            header {
                cursor: "{cursor}",
                user_select: "none",
                onmousedown: mousedown,
                onmouseover: move |_| { cursor.set("grab".into()) },
                h1 { { name } }
            }
            main {
            }
            footer {

            }
        }
    }
}
