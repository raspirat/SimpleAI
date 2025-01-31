use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::{ config::*, window::* };
use crate::utils::*;
use crate::config::*;

static CURRENT_PAGE_NAME: &str  = "new";
static STYLE: Asset = asset!("/assets/theme/pages/editor/index.css", CssAssetOptions::new().with_preload(true).with_minify(true) );

#[cfg(feature = "desktop")]
pub mod platform {

	use dioxus::desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
	use dioxus::desktop::muda::{};
	use dioxus::desktop::tao::{window::{Icon}};

	fn config() -> Config {
		use crate::config::ICON;

		Config::default()
			.with_close_behaviour(WindowCloseBehaviour::CloseWindow)
			.with_menu(None)
			.with_window(
				WindowBuilder::new()
					.with_inner_size(LogicalSize::new(1920, 1080))
					.with_transparent(true)
			)
	}

	use crate::platform::window::Window;
	pub fn window() -> Window
	{
		use crate::pages::editor::app;
		Window { app, config }
	}
}

#[cfg(feature = "web")]
crate::web_platform_window!(CURRENT_PAGE_NAME);

#[component]
pub fn Editor() -> Element {
	use crate::components::*;

	rsx! {
		head {
			document::Stylesheet {
				href: STYLE
			}
		}
		main {
			Divider
			{
				h1 {
					"hello"
				}
				aside {
					nav {}
					section {
						Search {}
					}
				}
			}
		}
	}
}
