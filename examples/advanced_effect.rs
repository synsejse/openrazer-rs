use openrazer::{devices::GenericMethods, DeviceManager, Error::UnsupportedCapability};
use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};
use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum Effect {
    BreathRandom,
    BreathSingle,
    BreathDual,
    BreathTriple,
    Reactive,
    Spectrum,
    Wave,
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(0, 7) {
            0 => Effect::BreathRandom,
            1 => Effect::BreathSingle,
            2 => Effect::BreathDual,
            3 => Effect::BreathTriple,
            4 => Effect::Reactive,
            5 => Effect::Spectrum,
            _ => Effect::Wave,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let manager = DeviceManager::new().await?;
    let devices = manager.get_connected_devices().await?;

    println!("Found {} Razer devices", devices.len());
    println!();

    manager.set_sync_effect(false).await?;

    for device in devices {
        let effect = random();
        println!("Setting {} to effect {:?}", device.name().await?, effect);
        match effect {
            Effect::BreathRandom => {
                device
                    .breath_effect()
                    .ok_or(UnsupportedCapability)?
                    .random()
                    .await?
            }
            Effect::BreathSingle => {
                device
                    .breath_effect()
                    .ok_or(UnsupportedCapability)?
                    .single(random())
                    .await?
            }
            Effect::BreathDual => {
                device
                    .breath_effect()
                    .ok_or(UnsupportedCapability)?
                    .dual(random(), random())
                    .await?
            }
            Effect::BreathTriple => {
                device
                    .breath_effect()
                    .ok_or(UnsupportedCapability)?
                    .triple(random(), random(), random())
                    .await?
            }
            Effect::Reactive => {
                device
                    .reactive_effect()
                    .ok_or(UnsupportedCapability)?
                    .medium(random())
                    .await?
            }
            Effect::Spectrum => {
                device
                    .spectrum_effect()
                    .ok_or(UnsupportedCapability)?
                    .set()
                    .await?
            }
            Effect::Wave => {
                device
                    .wave_effect()
                    .ok_or(UnsupportedCapability)?
                    .right()
                    .await?
            }
        }
    }

    Ok(())
}
