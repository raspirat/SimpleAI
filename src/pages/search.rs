use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::window::*;
use crate::utils::*;

static CURRENT_PAGE_NAME: &str  = "search";
static STYLE: Asset = asset!("/assets/theme/pages/search/index.css");

#[cfg(feature = "desktop")]
crate::desktop_platform_window!({
		Config::default()
			.with_close_behaviour(WindowCloseBehaviour::CloseWindow)
			.with_menu(None)
			.with_window(
				WindowBuilder::new()
					.with_transparent(true)
		)
});

#[cfg(feature = "web")]
crate::web_platform_window!(CURRENT_PAGE_NAME);

#[component]
pub fn Search() -> Element {
	use crate::components::*;
	rsx! {
		head {
			document::Stylesheet {
				href: STYLE
			}
		}
		main {
			Search {}
		}
	}
}