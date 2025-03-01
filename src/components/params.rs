use crate::utils;

use super::InternConnection;

#[derive(PartialEq, Props, Clone)]
pub struct InternRuntimeParam {
    pub param: utils::StrongParam,
    #[props(default = Signal::default())]
    pub connection: Signal<InternConnection>,
}
impl From<utils::StrongParam> for InternParam {
    fn from(param: utils::StrongParam) -> Self {
        let b = Self::builder().param(param);
        if let Ok(data) = intern.param.context.lock() {
            if let utils::ParamKind::Runtime { runtime } = data.kind {
                return b.connection(Signal::new(InternConnection::from(runtime.kind.clone())));
            }
        }
        b.build()
    }
}

#[sai_macros::element("component")]
pub fn RuntimeParam(style: String, intern: InternRuntimeParam) -> Element {
    rsx! {
        style { { style } }
        body {
            class: "Param",
            Connection { intern }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct InternStaticParam {
    pub param: utils::StrongParam,
}
impl From<utils::StrongParam> for InternParam {
    fn from(param: utils::StrongParam) -> Self {
        let b = Self::builder();
        b.param(param).build()
    }
}

#[sai_macros::element("component")]
pub fn StaticParam(style: String, intern: InternStaticParam) -> Element {
    rsx! {
        style { { style } }
        body {
            class: "Param",
        }
    }
}
