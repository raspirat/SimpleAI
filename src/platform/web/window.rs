use std::fmt::Debug;
use dioxus::web::Config;
use dioxus::dioxus_core::{Element};
use dioxus::prelude::*;
use crate::platform::WindowLike;

pub struct Window
{
	pub route: String,
}

impl Window
{
	pub fn new(route: &str) -> Self
	{
		Window { route: route.to_string() }
	}
}

impl WindowLike for Window
{
	fn open(&self) {
		_ = router().push(self.route.clone());
	}
}
