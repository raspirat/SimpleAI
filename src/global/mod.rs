pub(crate) mod context {
    use dioxus::html::MouseEvent;
    use dioxus::signals::*;
    use sai_backend::utils::*;
    pub(crate) static DRAG_NODE: GlobalSignal<Option<Node>> = Signal::global(|| None);
    use crate::components::InternConnection;
    pub(crate) static CONNECTION: GlobalSignal<Option<InternConnection>> = Signal::global(|| None);
}
