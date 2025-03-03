use sai_backend::utils::Node;

#[derive(PartialEq, Props, Clone)]
pub struct InternSearchResult {
    node: Node,
}

impl From<Node> for InternSearchResult {
    fn from(node: Node) -> Self {
        Self::builder().node(node).build()
    }
}

#[sai_macros::element("component")]
pub fn SearchResult(style: String, icons: Icons, intern: InternSearchResult) -> Element {
    let dragstart = move |_| *crate::global::context::DRAG_NODE.write() = Some(intern.node.clone());

    rsx! {
        style { {style} }
        div {
            draggable: true,
            class: "SearchResult",
            ondragstart: dragstart,
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
