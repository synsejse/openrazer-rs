//! Types for controlling the wave effect.

use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

/// Controls the wave effect.
#[derive(Debug, Clone)]
pub struct WaveEffect(pub(crate) CapabilityBase);

pub enum WaveDirection {
    Right = 1,
    Left = 2,
}

impl WaveEffect {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD: &'static str = "setWave";

    /// Sets the wave effect.
    pub async fn set(&self, direction: WaveDirection) -> Result<(), Error> {
        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::METHOD,
                vec![Value::Int32(direction as i32)],
            )
            .await
    }

    /// Sets the wave effect with `WaveDirection::Left`.
    pub async fn left(&self) -> Result<(), Error> {
        self.set(WaveDirection::Left).await
    }

    /// Sets the wave effect with `WaveDirection::Right`.
    pub async fn right(&self) -> Result<(), Error> {
        self.set(WaveDirection::Right).await
    }
}
