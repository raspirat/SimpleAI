use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::window::*;
use crate::utils::*;
use crate::pages::*;

static CURRENT_PAGE_NAME: &str  = "start";
static STYLE: Asset = asset!("/src/assets/theme/pages/start/index.css");

#[cfg(feature = "desktop")]
pub mod platform {
	use crate::pages::start::*;

	use dioxus::desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
	fn config() -> Config {
		let size = LogicalSize::new(480, 330);
		Config::new()
			.with_menu(None)
			.with_window(
				WindowBuilder::new()
					.with_max_inner_size(size)
					.with_min_inner_size(size)
					.with_resizable(true)
					.with_maximizable(false)
					.with_transparent(true)
					.with_title(CURRENT_PAGE_NAME)
			)
	}

	use crate::platform::window::Window;
	pub fn window() -> Window
	{
		Window { app, config }
	}
}

pub fn app() -> Element {
	let open_search = move |_| {
		search::platform::window().open();
	};

	let open_new = move |_| {
		new::platform::window().open();
	};

	rsx! {
		head {
			document::Stylesheet {
				href: STYLE
			}
		}
		main {
			article {
				section {
					button { class: "search", onclick: open_search, div { class: "icon search" } }
					button { class: "new", onclick: open_new, div { class: "icon new" } }
				}
			}
		}
	}
}