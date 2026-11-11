use crate::{db::create_post, Route};
use dioxus::prelude::*;

#[component]
pub fn CreatePost() -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut saving = use_signal(|| false);
    let mut message = use_signal(String::new);
    let mut saved = use_signal(|| false);

    rsx! {
        section { class: "page narrow-page",
            div { class: "page-heading",
                div {
                    span { class: "eyebrow", "WRITE TO SQLITE" }
                    h1 { "Create a post" }
                    p { class: "page-subtitle", "Just a title and description. No accounts, no extras." }
                }
            }

            form {
                class: "form-card",
                onsubmit: move |event| async move {
                    event.prevent_default();
                    if saving() {
                        return;
                    }
                    saved.set(false);
                    let new_title = title().trim().to_string();
                    let new_description = description().trim().to_string();
                    if new_title.is_empty() || new_description.is_empty() {
                        message.set("Please fill in both fields.".into());
                        return;
                    }

                    saving.set(true);
                    message.set(String::new());
                    match create_post(new_title, new_description).await {
                        Ok(()) => {
                            title.set(String::new());
                            description.set(String::new());
                            saved.set(true);
                        }
                        Err(error) => message.set(format!("Could not save post: {error}")),
                    }
                    saving.set(false);
                },
                div { class: "form-intro",
                    h2 { "New post" }
                    p { "Both fields are required." }
                }
                div { class: "field",
                    label { r#for: "post-title", "Title" }
                    input {
                        id: "post-title",
                        r#type: "text",
                        placeholder: "Give your post a title",
                        value: "{title}",
                        maxlength: "120",
                        oninput: move |event| {
                            title.set(event.value());
                            saved.set(false);
                        },
                    }
                }
                div { class: "field",
                    label { r#for: "post-description", "Description" }
                    textarea {
                        id: "post-description",
                        rows: "6",
                        placeholder: "What would you like to share?",
                        value: "{description}",
                        oninput: move |event| {
                            description.set(event.value());
                            saved.set(false);
                        },
                    }
                }

                if !message().is_empty() {
                    div { class: "notice error", "{message}" }
                }
                if saved() {
                    div { class: "notice success",
                        "Post saved! "
                        Link { to: Route::Posts {}, "View all posts" }
                    }
                }

                div { class: "form-actions",
                    Link { to: Route::Posts {}, class: "cancel-link", "Back to posts" }
                    button {
                        class: "primary-button",
                        r#type: "submit",
                        disabled: saving(),
                        if saving() { "Saving..." } else { "Save post" }
                    }
                }
            }
        }
    }
}
