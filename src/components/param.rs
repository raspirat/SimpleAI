use crate::utils;

use super::InternConnection;

#[derive(PartialEq, Props, Clone)]
pub struct InternParam {
    pub param: utils::StrongParam,
    #[props(default = Signal::default())]
    pub connection: Signal<Option<InternConnection>>,
}
impl From<utils::StrongParam> for InternParam {
    fn from(param: utils::StrongParam) -> Self {
        let b = Self::builder();
        b.param(param).build()
    }
}
#[sai_macros::element("component")]
pub fn Param(style: String, intern: InternParam) -> Element {
    use super::{Connection, InternConnection};

    let mut rendered_connection = use_signal(|| {
        rsx! {}
    });

    use_effect(move || {
        if let Some(intern) = (intern.connection)() {
            rendered_connection.set(rsx! { Connection { intern } });
        }
    });

    let mut rendered_intern = use_signal(|| rsx! {});

    let mounted = move |_| {
        if let NodeParamKind::Connected { param, .. } = &intern.param {
            match param {
                NodeParam::Named { name } => rendered_intern.set(rsx! {
                     h3 { { name.clone() } }
                }),
            }
        };
    };

    rsx! {
        style { { style } }
        body {
            onmounted: mounted,
            class: "Param",
                { rendered_connection }
                { rendered_intern }
        }
    }
}
