//! Types for controlling the reactve effect.

use super::CapabilityBase;
use crate::{Color, Error};
use dbus_message_parser::Value;

/// Controls the reactive effect.
#[derive(Debug, Clone)]
pub struct ReactiveEffect(pub(crate) CapabilityBase);

/// The effect's speed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Speed {
    /// The reaction stays for 500ms.
    Fast = 1,
    /// The reaction stays for 1000ms.
    Medium = 2,
    /// The reaction stays for 1500ms.
    Slow = 3,
    /// The reaction stays for 2000ms.
    VerySlow = 4,
}

impl ReactiveEffect {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD: &'static str = "setReactive";

    /// Sets the reactive effect.
    pub async fn set(&self, (red, green, blue): Color, speed: Speed) -> Result<(), Error> {
        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::METHOD,
                vec![
                    Value::Byte(red),
                    Value::Byte(green),
                    Value::Byte(blue),
                    Value::Byte(speed as u8),
                ],
            )
            .await
    }

    /// Sets the reactive effect with `Speed::Fast`.
    pub async fn fast(&self, color: Color) -> Result<(), Error> {
        self.set(color, Speed::Fast).await
    }

    /// Sets the reactive effect with `Speed::Medium`.
    pub async fn medium(&self, color: Color) -> Result<(), Error> {
        self.set(color, Speed::Medium).await
    }

    /// Sets the reactive effect with `Speed::Slow`.
    pub async fn slow(&self, color: Color) -> Result<(), Error> {
        self.set(color, Speed::Slow).await
    }

    /// Sets the reactive effect with `Speed::VerySlow`.
    pub async fn very_slow(&self, color: Color) -> Result<(), Error> {
        self.set(color, Speed::VerySlow).await
    }
}
