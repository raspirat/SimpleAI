#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // for windows

mod app;
mod page;
mod theme;

fn main() {
    app::run();
}
