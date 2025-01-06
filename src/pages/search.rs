use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::window::*;
use crate::utils::*;

static CURRENT_PAGE_NAME: &str  = "search";
static STYLE: Asset = asset!("/src/assets/theme/pages/search/index.css");

#[cfg(feature = "desktop")]
pub mod platform {

	use dioxus::desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
	use dioxus::desktop::muda::Menu;

	fn config() -> Config {
		Config::default()
			.with_close_behaviour(WindowCloseBehaviour::CloseWindow)
			.with_menu(None)
			.with_window(
				WindowBuilder::new()
					.with_inner_size(LogicalSize::new(680, 1000))
					.with_resizable(false)
					.with_maximizable(false)
					.with_transparent(true)
			)
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
		head {
			document::Stylesheet {
				href: STYLE
			}
		}
		header {
			input { type: "search", placeholder: "search" }
			button { "your nodes" }
			button { "installed nodes" }
			button { "profiles" }
		}
		main {
			div { }
		}
	}
}