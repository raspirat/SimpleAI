pub mod window;
pub mod config;
pub mod macros;

pub fn launch()
{
	use dioxus::desktop::{ Config, launch::launch_virtual_dom };
	let start_window = crate::pages::start::platform::window();
	launch_virtual_dom(
	  start_window.virtual_dom(),
		start_window.config()
	);
}