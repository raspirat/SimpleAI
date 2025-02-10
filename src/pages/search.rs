use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::platform::window::*;

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

	rsx! {
		main {
			Search {}
		}
	}
}
