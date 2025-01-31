use dioxus::prelude::*;
// ─────────────────────────────────────────────────────────────────────────────────────
use crate::utils::*;
use crate::pages::*;
use crate::platform::*;

static CURRENT_PAGE_NAME: &str  = "start";
static STYLE: Asset = asset!("/assets/theme/pages/start/index.css");

#[cfg(feature = "desktop")]
crate::desktop_platform_window!({
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
                .with_title("My App")
        )
});

#[cfg(feature = "web")]
crate::web_platform_window!(CURRENT_PAGE_NAME);

#[component]
pub fn Start() -> Element {
	let open_search = move |_| {
		search::platform::window().open();
	};

	let open_new = move |_| {
		new::platform::window().open();
	};

	let open_editor = move |_| {
		editor::platform::window().open();
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
					button { class: "editor", onclick: open_editor, div { class: "icon", id: "editor" } }
				}
			}
		}
	}
}