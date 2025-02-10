pub mod window;
pub use window::*;

pub fn launch() {
    use dioxus::desktop::{launch::launch_virtual_dom, Config};
    let start_window = crate::pages::start::StartWindow();
    launch_virtual_dom(start_window.virtual_dom(), start_window.config());
}
