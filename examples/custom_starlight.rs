use futures::future::pending;
use openrazer::{
    capabilities::custom_effect::Frame,
    devices::{Device, GenericMethods},
    DeviceManager,
    Error::UnsupportedCapability,
};
use palette::{encoding::Srgb, Hsv, IntoColor};
use rand::{thread_rng, Rng};
use std::{
    collections::HashSet,
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::Mutex,
    time::{delay_for, interval},
};

async fn starlight_key(
    (row, column): (usize, usize),
    frame: Arc<Mutex<Frame>>,
) -> Result<(), openrazer::Error> {
    let hue = thread_rng().gen_range(0.0, 360.0);
    let start_time = Instant::now();
    let fade_time = Duration::from_secs(2);

    loop {
        let elapsed = start_time.elapsed();
        if start_time.elapsed() < fade_time {
            let value = 1.0 - elapsed.as_secs_f64() / fade_time.as_secs_f64();
            let color = Hsv::new(hue, 1.0, value).into_rgb::<Srgb>();
            let color = color.into_format::<u8>().into_components();
            frame.lock().await[(row, column)] = color;
            delay_for(Duration::from_secs_f64(1.0 / 60.0)).await;
        } else {
            frame.lock().await[(row, column)] = (0, 0, 0);
            break Ok(());
        }
    }
}

async fn starlight_effect(device: Device) -> Result<(), openrazer::Error> {
    let dimensions = device
        .matrix_dimensions()
        .ok_or(UnsupportedCapability)?
        .get()
        .await?;
    let custom_effect = device.custom_effect().ok_or(UnsupportedCapability)?;

    let active = Arc::new(Mutex::new(HashSet::with_capacity(
        dimensions.rows * dimensions.columns,
    )));

    custom_effect.set().await?;
    let frame = custom_effect.frame(dimensions);
    frame.draw().await?;
    let frame = Arc::new(Mutex::new(frame));

    {
        let frame = Arc::clone(&frame);
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs_f64(1.0 / 60.0));
            loop {
                interval.tick().await;
                frame.lock().await.draw().await.unwrap();
            }
        });
    }

    loop {
        let (row, column) = {
            let mut rng = thread_rng();
            (
                rng.gen_range(0, dimensions.rows),
                rng.gen_range(0, dimensions.columns),
            )
        };

        if !active.lock().await.contains(&(row, column)) {
            let active = Arc::clone(&active);
            let frame = Arc::clone(&frame);

            active.lock().await.insert((row, column));
            tokio::spawn(async move {
                starlight_key((row, column), frame).await.unwrap();
                active.lock().await.remove(&(row, column));
            });
        }

        delay_for(Duration::from_millis(100)).await;
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
        if !device.capabilities().custom_effect.is_available() {
            println!(
                "Skipping device {} ({})",
                device.name().await?,
                device.serial()
            );
            continue;
        }

        tokio::spawn(async move {
            starlight_effect(device).await.unwrap();
        });
    }

    pending().await
}
