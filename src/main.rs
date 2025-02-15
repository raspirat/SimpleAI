mod assets;
mod components;
mod config;
mod pages;
mod platform;

pub(crate) use platform::*;

fn main() {
    platform::launch();
}
