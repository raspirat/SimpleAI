mod assets;
mod components;
mod config;
mod global;
mod pages;
mod platform;

fn main() {
    // dioxus::logger::init(dioxus::logger::tracing::Level::).expect("failed to init logger");
    platform::launch();
}
