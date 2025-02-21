mod assets;
mod components;
mod config;
mod global;
mod pages;
mod platform;
mod utils;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("failed to init logger");
    platform::launch();
}
