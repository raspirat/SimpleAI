mod assets;
// mod components;
mod config;
mod pages;
mod platform;

pub(crate) use assets::dirs::current_theme_dir;

fn main() {
    use dioxus::logger::*;
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("failed to init logger");

    tracing::debug!("Starting!");

    platform::launch();
}
