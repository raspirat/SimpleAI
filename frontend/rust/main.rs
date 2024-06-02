#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // for windows

mod themes;
mod errors;


fn main() {
  println!("Application starting...");
  let app = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  let start_page = tauri::WindowBuilder::new(
    &app,
    "start_page",
    tauri::WindowUrl::App("pages/start_page/html/index.html".into())
  ).build().expect("failed to build window");

  start_page.open_devtools();

  app.run(|_, _| {});
}
