# openrazer

Tokio 1 / zbus 6 asynchronous OpenRazer bindings. This is a history-preserving
MIT-licensed continuation of <https://gitlab.com/SnejUgal/openrazer-rs> by SnejUgal.
The original copyright and license are preserved in LICENSE.

Version 0.2 is a breaking migration from the 2020 Tokio 0.2 API. Use `Client`
with an existing zbus connection or `Client::connect()`. See `examples/enumerate.rs`
for read-only enumeration. Requires Rust 1.99 and a Tokio runtime.

Development: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --all-targets`, `cargo test --doc`, `cargo doc --no-deps`.
Tests must use synthetic fixtures or private buses; never real hardware setters.

All 225 audited current method signatures are available as typed descriptors in
`openrazer::methods`. `Endpoint::call` checks exact introspection signatures before
sending a call. Unsupported methods and failed reads remain distinct errors.

See [usage and migration](docs/MIGRATION.md) for examples and the razerr-rs API map,
and [source audit](docs/AUDIT.md) for coverage, provenance and known limits.

```rust,no_run
use openrazer::{Client, methods::device_dpi};
# async fn example() -> Result<(), openrazer::Error> {
let client = Client::connect().await?;
for serial in client.serials().await? {
    let device = client.device(&serial).await?;
    if device.capabilities().supports(&device_dpi::GET_DPI) {
        let (axes,) = device.call(&device_dpi::GET_DPI, &()).await?;
        println!("{serial}: {axes:?}");
    }
}
# Ok(()) }
```
