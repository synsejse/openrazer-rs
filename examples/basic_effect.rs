use openrazer::{devices::GenericMethods, DeviceManager, Error::UnsupportedCapability};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let manager = DeviceManager::new().await?;
    let devices = manager.get_connected_devices().await?;

    println!("Found {} Razer devices", devices.len());
    println!();

    manager.set_sync_effect(false).await?;

    for device in devices {
        println!("Setting {} to wave", device.name().await?);
        device
            .wave_effect()
            .ok_or(UnsupportedCapability)?
            .right()
            .await?;
    }

    Ok(())
}
