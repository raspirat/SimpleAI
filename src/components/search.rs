use dioxus::prelude::*;
static STYLE: Asset = asset!("/assets/theme/components/search/index.css");

#[component]
pub fn Search() -> Element {
	use crate::components::*;
	rsx! {
		document::Stylesheet { href: STYLE }
		div {
			class: "Search",
			header {
				input { type: "search", placeholder: "search" }
				// nav {
				// 	button { "your nodes" }
				// 	button { "installed nodes" }
				// 	button { "profiles" }
				// }
			}
			main {
				div { class: "spacer" }
				section { class: "results",
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
					SearchResult {}
				}
			}
		}
	}
}