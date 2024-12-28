use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::window::*;
use crate::utils::*;

static CURRENT_PAGE_NAME: &str  = "search";
static STYLE: Asset = asset!("/assets/theme/pages/search/index.css");

#[cfg(feature = "desktop")]
pub mod platform {

	use dioxus::desktop::{Config, WindowBuilder};
	use dioxus::desktop::muda::Menu;

	fn config() -> Config {
		Config::default()
			.with_menu(None)
	}

	use crate::platform::window::Window;
	pub fn window() -> Window
	{
		use crate::pages::search::app;
		Window { app, config }
	}
}

fn app() -> Element {
	rsx! {
		document::Stylesheet {
			href: STYLE
		}
		article {
			section {
				h1 {
					"SEARCH"
				}
			}
		}
	}
}
