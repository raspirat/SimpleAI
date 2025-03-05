use sai_backend::utils::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InternRuntimeParam {
    pub param: StrongParam,
    pub connection: Signal<super::InternConnection>,
}
impl From<StrongParam> for InternRuntimeParam {
    fn from(param: StrongParam) -> Self {
        let b = Self::builder().param(param.clone());
        if let Ok(data) = param.context.try_lock() {
            if let ParamKind::Runtime { kind, .. } = &data.kind {
                return b
                    .connection(Signal::new(super::InternConnection::from(kind.clone())))
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
            super::Connection { intern: (intern.connection)() }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct InternStaticParam {
    pub param: StrongParam,
}
impl From<StrongParam> for InternStaticParam {
    fn from(param: StrongParam) -> Self {
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
