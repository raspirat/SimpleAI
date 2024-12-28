use std::fmt::Debug;
use dioxus::desktop::{Config};
use dioxus::desktop::launch::launch_virtual_dom;
use dioxus::dioxus_core::{Element};
use dioxus::prelude::VirtualDom;

pub struct Window
{
	pub(crate) app: fn() -> Element,
	pub(crate) config: fn() -> Config
}

impl Window
{
	pub fn new(app: fn() -> Element, config: fn() -> Config) -> Self
	{
		Window { app, config }
	}
	pub fn config(&self) -> Config
	{
		(&self.config)()
	}
	pub fn virtual_dom(&self) -> VirtualDom
	{
		VirtualDom::new(self.app)
	}
	pub fn open(&self)
	{
		dioxus::desktop::window()
			.new_window(
				self.virtual_dom(),
				self.config()
			);
	}
}
