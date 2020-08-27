# `openrazer-rs`

Asynchronous bindings to the OpenRazer daemon, written in pure Rust. The API is
somewhat similar `openrazer` Python module. Here's an example of applying the
wave effect to all devices:

```rust
use openrazer::{devices::GenericMethods, DeviceManager, Error::UnsupportedCapability};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let manager = DeviceManager::new().await?;
    manager.sync_effects().disable().await?;

    let devices = manager.devices().await?;
    println!("Found {} Razer devices", devices.len());

    for device in devices {
        println!("Setting wave effect for {}", device.name().await?);
        device.wave_effect().ok_or(UnsupportedCapability)?.right().await?;
    }

    Ok(())
}
```

There are also other [examples] in the repository.

[examples]: ./examples

## Disclaimer

I wrote this crate to fulfill my own need: change lighting mode on my Razer
Huntsman keyboard. This keyboard is the only Razer device I have and can test
this crate on, so I didn't add bindings to features that my keyboard doesn't
support. That said, implemented features will work on any device that supports
them, and it's easy to add new bindings, so merge requests are welcome!
