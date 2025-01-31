use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::{ config::*, window::* };
use crate::utils::*;

static CURRENT_PAGE_NAME: &str  = "new";
static STYLE: Asset = asset!("/assets/theme/pages/new/index.css");

#[cfg(feature = "desktop")]
pub mod platform {

	use dioxus::desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
	use dioxus::desktop::muda::Menu;
	use dioxus::desktop::tao::dpi::Size;
	use dioxus::desktop::tao::dpi::Size::Logical;

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
		use crate::pages::new::app;
		Window { app, config }
	}
}

#[cfg(feature = "web")]
crate::web_platform_window!(CURRENT_PAGE_NAME);

#[component]
pub fn New() -> Element {
	use crate::components::*;

	rsx! {
		document::Stylesheet {
			href: STYLE
		}
		main {
			form {
				LabeledBox {
					label { for: "name", "node name" }
					input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
				}
				LabeledBox {
					input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
					label { for: "name", "project name" }
				}
				input { type: "text" }
				LabeledBox {
					label { for: "name", "project name" }
					input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
				}
				LabeledBox {
					input { id: "name", name: "name", type: "text", required: "true", placeholder: "SampleProject"}
					label { for: "name", "project name" }
				}
				input { type: "file" }
				input { type: "range" }

				input { type: "list", list: "options" }

				datalist {
					id: "options",
					option { value: "1", "1" }
					option { value: "2", "2" }
					option { value: "3", "3" }
				}

				button { type: "submit", "create" }
			}
		}
	}
}
