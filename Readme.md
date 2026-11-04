# openrazer

Tokio 1 / zbus 6 asynchronous OpenRazer bindings. This is a history-preserving
MIT-licensed continuation of https://gitlab.com/SnejUgal/openrazer-rs by SnejUgal.
The original copyright and license are preserved in LICENSE.

Version 0.2 is a breaking migration from the 2020 Tokio 0.2 API. Use `Client`
with an existing zbus connection or `Client::connect()`. See `examples/enumerate.rs`
for read-only enumeration. Requires Rust 1.99 and a Tokio runtime.

Development: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --all-targets`, `cargo test --doc`, `cargo doc --no-deps`.
Tests must use synthetic fixtures or private buses; never real hardware setters.
