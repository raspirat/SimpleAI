#[sai_macros::element("component")]
pub fn NamedParam(style: String, name: String) -> Element {
    rsx! {
        style { { style } }
        body {
            class: "NamedParam Param Named",
            div { class: "input connection" }
            h3 { { name } }
            div { class: "output connection" }
        }
    }
}
