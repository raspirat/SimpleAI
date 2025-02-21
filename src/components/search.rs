#[sai_macros::element("component")]
pub fn Search(style: String, icons: Icons) -> Element {
    use crate::components::*;
    use crate::utils::{Node, NodeParam};

    let mut search_results = use_signal(Vec::<InternSearchResult>::new);

    let onchange = move |e: FormEvent| {
        dioxus::logger::tracing::debug!("submit");
        search_results.push(InternSearchResult::from(Node {
            name: "sample".into(),
            params: vec![NodeParam::Named {
                name: "sample".into(),
            }],
        }));
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
