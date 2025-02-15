#[sai_macros::element("component")]
pub fn Viewport(children: Element) -> Element {
    rsx! {
        main {
            { children }
        }
    }
}
