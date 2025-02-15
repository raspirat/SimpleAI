pub mod window;
pub use window::*;

use crate::pages::*;

use dioxus::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Start {},
    #[route("/search")]
    Search {},
    #[route("/new")]
    New {},
    #[route("/editor")]
    Editor {},
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

pub fn launch() {
    use dioxus::web::{launch::launch_virtual_dom, Config};
    launch_virtual_dom(VirtualDom::new(App), Config::default());
}
