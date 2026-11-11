use crate::{db::list_posts, Route};
use dioxus::prelude::*;

#[component]
pub fn Posts() -> Element {
    let mut posts = use_resource(|| async { list_posts().await.map_err(|error| error.to_string()) });

    rsx! {
        section { class: "page",
            div { class: "page-heading",
                div {
                    span { class: "eyebrow", "FULLSTACK DEMO" }
                    h1 { "Your posts" }
                    p { class: "page-subtitle", "A small feed stored in SQLite and loaded through a Rust server function." }
                }
                Link { to: Route::CreatePost {}, class: "primary-button",
                    span { "+" }
                    " New post"
                }
            }

            div { class: "info-strip",
                div { class: "info-icon", "▤" }
                div {
                    strong { "Persistent storage" }
                    p { "Posts are saved in posts.db on the server." }
                }
                span { class: "tech-tag", "SQLite" }
            }

            div { class: "section-heading",
                h2 { "Recent posts" }
                button { class: "text-button", onclick: move |_| posts.restart(), "↻ Refresh" }
            }

            if let Some(result) = &*posts.read() {
                match result {
                    Ok(items) if items.is_empty() => rsx! {
                        div { class: "empty-state",
                            div { class: "empty-icon", "✎" }
                            h3 { "No posts yet" }
                            p { "Create your first post to see the database in action." }
                            Link { to: Route::CreatePost {}, class: "secondary-button", "Create a post" }
                        }
                    },
                    Ok(items) => rsx! {
                        div { class: "post-count", "{items.len()} posts in the database" }
                        div { class: "post-list",
                            for post in items.iter() {
                                article { class: "post-card", key: "{post.id}",
                                    div { class: "post-number", "#{post.id}" }
                                    h3 { "{post.title}" }
                                    p { "{post.description}" }
                                }
                            }
                        }
                    },
                    Err(error) => rsx! {
                        div { class: "notice error", "Could not load posts: {error}" }
                    },
                }
            } else {
                div { class: "empty-state", "Loading posts..." }
            }
        }
    }
}
