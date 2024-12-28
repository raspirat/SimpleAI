use dioxus::prelude::*;

mod platform;
mod components;
mod pages;
mod utils;

fn main() {
	platform::launch(pages::start::platform::window());
}