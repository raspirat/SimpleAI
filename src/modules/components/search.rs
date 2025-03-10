use sai_backend::utils::prelude::QueryFilter;

#[sai_macros::element("component")]
pub fn Search(style: String, icons: Icons) -> Element {
    use crate::components::{InternSearchResult, SearchResult};
    use sai_backend::nms::query::*;

    let mut search_results = use_signal(Vec::<InternSearchResult>::new);

    let onchange = move |e: FormEvent| {
        let mut results = query(vec![QueryFilter::Name { name: e.value() }]).tree;
        search_results.extend(
            results
                .iter_mut()
                .map(|result| InternSearchResult::from(result.clone()))
                .collect::<Vec<InternSearchResult>>(),
        );
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
