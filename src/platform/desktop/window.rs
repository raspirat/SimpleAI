pub use crate::platform::WindowLike;
use dioxus::desktop::Config;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::fmt::Debug;

pub struct Window {
    pub(crate) app: fn() -> Element,
    pub(crate) config: fn() -> Config,
}

impl Window {
    pub fn new(app: fn() -> Element, config: fn() -> Config) -> Self {
        Window { app, config }
    }
    pub fn config(&self) -> Config {
        (&self.config)()
    }
    pub fn virtual_dom(&self) -> VirtualDom {
        VirtualDom::new(self.app)
    }
}

impl WindowLike for Window {
    fn open(&self) {
        _ = dioxus::desktop::window().new_window(self.virtual_dom(), self.config())
    }
}
