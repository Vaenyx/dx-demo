use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut collapsed = use_signal(|| false);

    rsx! {
        div { class: if collapsed() { "app-shell is-collapsed" } else { "app-shell" },
            aside { class: "sidebar",
                button {
                    class: "sidebar-toggle",
                    title: "Toggle sidebar",
                    r#type: "button",
                    onclick: move |_| collapsed.set(!collapsed()),
                    if collapsed() { "›" } else { "‹" }
                }
                Link { to: Route::Posts {}, class: "brand",
                    span { class: "brand-mark", "D" }
                    span { class: "brand-name", "dioxus", span { "demo" } }
                }

                div { class: "nav-caption", "PLAYGROUND" }
                nav { class: "nav-list",
                    Link {
                        to: Route::Posts {},
                        class: "nav-link",
                        active_class: "active",
                        span { class: "nav-icon", "▦" }
                        span { class: "nav-text", "Posts" }
                    }
                    Link {
                        to: Route::CreatePost {},
                        class: "nav-link",
                        active_class: "active",
                        span { class: "nav-icon", "+" }
                        span { class: "nav-text", "Create post" }
                    }
                    Link {
                        to: Route::Echo {},
                        class: "nav-link",
                        active_class: "active",
                        span { class: "nav-icon", "↩" }
                        span { class: "nav-text", "Echo" }
                    }
                }
                div { class: "sidebar-footer",
                    span { class: "status-dot" }
                    span { class: "footer-text", "Rust · Dioxus · SQLite" }
                }
            }

            main { class: "main-content",
                Outlet::<Route> {}
            }
        }
    }
}
