use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::window::*;
use crate::utils::*;
use crate::pages::*;
use crate::pages::start::platform::window;

static CURRENT_PAGE_NAME: &str  = "start";
static STYLE: Asset = asset!("/assets/theme/pages/start/index.css");

#[cfg(feature = "desktop")]
pub mod platform {

	use dioxus::desktop::Config;
	fn config() -> Config {
		Config::default()
	}

	use crate::platform::window::Window;
	pub fn window() -> Window
	{
		use crate::pages::start::app;
		Window { app, config }
	}
}

pub fn app() -> Element {
	let open_search = move |_| {
		search::platform::window().open();
	};

	let close_search = move |_| {
	};

	rsx! {
		document::Stylesheet {
			href: STYLE
		}
		article {
			section {
				button { class: "search", onclick: open_search, div { class: "icon search" } }
				button { class: "new", onclick: close_search, div { class: "icon new" } }
			}
		}
	}
}
