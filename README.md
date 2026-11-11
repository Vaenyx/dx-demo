# Dioxus demo

A small Dioxus 0.7 fullstack web app with a responsive sidebar, persistent SQLite posts and a server-function echo chat. No login or author names.

## Run

Install Rust and the Dioxus CLI if necessary:

```bash
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --locked
```

Start the fullstack application from this directory:

```bash
dx serve --web
```

Open the address shown by `dx` (typically http://127.0.0.1:8080).

## Demo

1. **Posts:** See posts loaded from SQLite. Use **Refresh** to request the list again.
2. **Create post:** Enter a title and description and save; go back to Posts to see it.
3. **Echo:** Send a message. A Rust server function echoes it back into the conversation.
4. Click the small sidebar arrow to collapse/expand it on desktop. On tablets it shrinks to icons; on phones it becomes a bottom navigation bar.

Cargo regenerates `Cargo.lock` on the first build. The `posts.db` file is created automatically in the directory where the server is started. Stop the server and delete it to reset your demo. Posts are kept after restarting the app; echo messages only live in the currently opened page.

## Relevant code

- `src/main.rs` — routes and app entry point
- `src/views/navbar.rs` — responsive sidebar
- `src/views/posts.rs` — database feed
- `src/views/create.rs` — post form
- `src/views/echo.rs` — echo conversation and server function
- `src/db.rs` — SQLite connection, schema and server functions
- `assets/styling/main.css` — all styling and breakpoints

WebAssembly runs the UI in the browser. The Rust server handles SQLite and the echo API.
