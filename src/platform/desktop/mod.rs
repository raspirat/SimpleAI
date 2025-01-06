use dioxus::desktop::{Config, WindowCloseBehaviour};
use dioxus::dioxus_core::VirtualDom;

pub mod window;

pub fn launch(start_window: window::Window)
{
	use dioxus::{
		dioxus_core::VirtualDom,
		desktop::launch::launch_virtual_dom
	};
	launch_virtual_dom(
		VirtualDom::new(start_window.app),
		start_window.config()
	);
}