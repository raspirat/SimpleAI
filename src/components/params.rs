use super::*;
use sai_backend::utils;

#[derive(PartialEq, Props, Clone)]
pub struct InternRuntimeParam {
    pub param: utils::StrongParam,
    pub connection: Signal<InternConnection>,
}
impl From<utils::StrongParam> for InternRuntimeParam {
    fn from(param: utils::StrongParam) -> Self {
        let b = Self::builder().param(param.clone());
        if let Ok(data) = param.context.lock() {
            if let utils::ParamKind::Runtime { runtime } = &data.kind {
                return b
                    .connection(Signal::new(InternConnection::from(runtime.kind.clone())))
                    .build();
            }
        }
        panic!("Couldn't create Param");
    }
}

#[sai_macros::element("component")]
pub fn RuntimeParam(style: String, intern: InternRuntimeParam) -> Element {
    rsx! {
        style { { style } }
        body {
            class: "Param",
            Connection { intern: (intern.connection)() }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct InternStaticParam {
    pub param: utils::StrongParam,
}
impl From<utils::StrongParam> for InternStaticParam {
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
