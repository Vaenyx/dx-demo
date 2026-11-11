use dioxus::prelude::*;
use views::{CreatePost, Echo, Navbar, Posts};

mod db;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Posts {},
        #[route("/create")]
        CreatePost {},
        #[route("/echo")]
        Echo {},
}

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        document::Title { "Dioxus Demo" }
        Router::<Route> {}
    }
}
