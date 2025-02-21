use crate::utils::NodeParam;

#[derive(PartialEq, Props, Clone)]
pub struct InternParam {
    param: NodeParam,
}
impl From<NodeParam> for InternParam {
    fn from(param: NodeParam) -> Self {
        Self::builder().param(param).build()
    }
}
#[sai_macros::element("component")]
pub fn Param(style: String, intern: InternParam) -> Element {
    // let rendered_intern = use_signal(|| rsx! { h1 { "sample" } });
    let rendered_intern = use_signal(|| match intern.param {
        NodeParam::Named { name } => {
            rsx! { h3 { { name } } }
        }
    });

    rsx! {
        style { { style } }
        body {
            class: "NamedParam Param Named",
            div { class: "input connection" }
            { rendered_intern }
            div { class: "output connection" }
        }
    }
}
