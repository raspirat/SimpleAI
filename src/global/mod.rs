pub(crate) mod context {
    use crate::utils::*;
    use dioxus::signals::*;
    pub(crate) static DRAG_NODE: GlobalSignal<Option<Node>> = Signal::global(|| None);
}
