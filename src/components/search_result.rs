use dioxus::prelude::*;

static STYLE: Asset = asset!("/assets/theme/components/search_result/index.css");

#[component]
pub fn SearchResult() -> Element {
	rsx! {
		document::Stylesheet { href: STYLE }
		div {
			class: "SearchResult",
			div {
				class: "wrapper items",
				h3 { span { id: "name", "SampleNode" } }

				div {
						class: "wrapper",
					div {
						class: "wrapper a",
						a { id: "website", "www.sne.com" }
					}
					div {
						class: "wrapper i",

						div { id: "open", class: "icon" }
						div { id: "favorite", class: "icon" }
					}
				}
			}
			h5 { id: "version", "12.23.1" }
			p { id: "description",
				{
					"This is a sample description. Don't take it seriously. It describes a node in need of documentation.
          I'll just write more to see how the description looks in html with css..."
				}
			}
			address {
				id: "author",
				"sertgrc"
			}
		}
	}
}