#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // for windows

mod themes;
mod errors;


fn main() {
  println!("Application starting...");
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
