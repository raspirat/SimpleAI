pub(crate) mod context {
    use dioxus::signals::*;
    use sai_backend::utils::prelude::StrongNode;
    pub(crate) static DRAG_NODE: GlobalSignal<Option<StrongNode>> = Signal::global(|| None);
    use crate::components::InternConnection;
    pub(crate) static CONNECTION: GlobalSignal<Option<InternConnection>> = Signal::global(|| None);
}
