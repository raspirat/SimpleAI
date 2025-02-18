#[sai_macros::element("component")]
pub fn Search(style: String, icons: Icons) -> Element {
	use crate::components::*;
	rsx! {
		style { { style } }
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
				}
			}
		}
	}

}
