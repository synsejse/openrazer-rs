use openrazer::{devices::GenericMethods, DeviceManager};
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
        let capabilities = device.capabilities();
        if !(capabilities.breath_effect != Default::default()
            || capabilities.reactive_effect
            || capabilities.spectrum_effect
            || capabilities.wave_effect)
        {
            println!(
                "Device {} doesn't support any of the effects",
                device.name().await?
            );
            continue;
        }

        let effect = loop {
            let effect = random();
            match effect {
                Effect::BreathSingle if capabilities.breath_effect.single => break effect,
                Effect::BreathDual if capabilities.breath_effect.dual => break effect,
                Effect::BreathTriple if capabilities.breath_effect.triple => break effect,
                Effect::Reactive if capabilities.reactive_effect => break effect,
                Effect::Spectrum if capabilities.spectrum_effect => break effect,
                Effect::Wave if capabilities.wave_effect => break effect,
                _ => (),
            }
        };
        println!("Setting {} to effect {:?}", device.name().await?, effect);

        match effect {
            Effect::BreathRandom => device.breath_effect().unwrap().random().await?,
            Effect::BreathSingle => device.breath_effect().unwrap().single(random()).await?,
            Effect::BreathDual => {
                device
                    .breath_effect()
                    .unwrap()
                    .dual(random(), random())
                    .await?
            }
            Effect::BreathTriple => {
                device
                    .breath_effect()
                    .unwrap()
                    .triple(random(), random(), random())
                    .await?
            }
            Effect::Reactive => device.reactive_effect().unwrap().medium(random()).await?,
            Effect::Spectrum => device.spectrum_effect().unwrap().set().await?,
            Effect::Wave => device.wave_effect().unwrap().right().await?,
        }
    }

    Ok(())
}
