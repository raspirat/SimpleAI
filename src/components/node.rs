#[sai_macros::element("component")]
pub fn Node(style: String) -> Element {
    rsx! {
        style { { style } }
        header {

        }
        main {

        }
        footer {

        }
    }
}
