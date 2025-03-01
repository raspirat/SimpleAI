#[sai_macros::element("component")]
pub fn Search(style: String, icons: Icons) -> Element {
    use crate::components::{InternSearchResult, SearchResult};
    use crate::utils::*;

    let mut search_results = use_signal(Vec::<InternSearchResult>::new);

    let onchange = move |e: FormEvent| {
        dioxus::logger::tracing::debug!("submit");
        // search_results.push();
    };

    let rendered_search_results = search_results
        .iter()
        .map(|intern| rsx! { SearchResult { intern: intern.clone() } });

    rsx! {
        style { { style } }
        div {
            class: "Search",
            header {
                input {
                    onchange: onchange,
                    type: "search",
                    placeholder: "search"
                }
                // nav {
                // 	button { "your nodes" }
                // 	button { "installed nodes" }
                // 	button { "profiles" }
                // }
            }
            main {
                div { class: "spacer" }
                section { class: "results" }
            {rendered_search_results}
            }
        }
    }
}
