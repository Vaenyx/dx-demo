use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Message {
    text: String,
    from_server: bool,
}

#[component]
pub fn Echo() -> Element {
    let mut input = use_signal(String::new);
    let mut messages = use_signal(Vec::<Message>::new);
    let mut sending = use_signal(|| false);
    let mut error = use_signal(String::new);

    rsx! {
        section { class: "page narrow-page",
            div { class: "page-heading",
                div {
                    span { class: "eyebrow", "SERVER FUNCTION" }
                    h1 { "Echo" }
                    p { class: "page-subtitle", "Send a message to the Rust backend. It sends the exact same text back." }
                }
            }

            div { class: "chat-card",
                div { class: "chat-header",
                    span { class: "chat-avatar", "↩" }
                    div {
                        h2 { "Echo server" }
                        p { "Your current conversation" }
                    }
                    span { class: "online-status", "Online" }
                }

                div { class: "chat-history",
                    if messages.read().is_empty() {
                        div { class: "chat-empty",
                            div { class: "chat-empty-icon", "↩" }
                            h3 { "Say hello!" }
                            p { "Your conversation will appear here." }
                        }
                    }
                    for message in messages.read().iter() {
                        div {
                            class: if message.from_server { "chat-line server" } else { "chat-line user" },
                            div { class: "chat-bubble", "{message.text}" }
                            span { class: "chat-label", if message.from_server { "Server" } else { "You" } }
                        }
                    }
                    if sending() {
                        div { class: "chat-line server",
                            div { class: "chat-bubble pending", "Echoing..." }
                        }
                    }
                }

                if !error().is_empty() {
                    div { class: "chat-error", "{error}" }
                }

                form {
                    class: "chat-compose",
                    onsubmit: move |event| async move {
                        event.prevent_default();
                        let text = input().trim().to_string();
                        if text.is_empty() || sending() {
                            return;
                        }
                        input.set(String::new());
                        error.set(String::new());
                        messages.write().push(Message { text: text.clone(), from_server: false });
                        sending.set(true);
                        match echo_server(text).await {
                            Ok(response) => messages.write().push(Message { text: response, from_server: true }),
                            Err(err) => error.set(format!("Could not reach server: {err}")),
                        }
                        sending.set(false);
                    },
                    input {
                        r#type: "text",
                        placeholder: "Write a message...",
                        value: "{input}",
                        oninput: move |event| input.set(event.value()),
                    }
                    button { class: "primary-button", r#type: "submit", disabled: sending(), "Send ↗" }
                }
            }
        }
    }
}

#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String> {
    Ok(input)
}
