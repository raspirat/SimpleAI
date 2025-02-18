use dioxus::signals::*;
pub(crate) static DRAG_CONTEXT: GlobalSignal<String> = Signal::global(String::new);
