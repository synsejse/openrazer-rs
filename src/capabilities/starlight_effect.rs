//! Types for controlling the reactve effect.

use super::{available::StarlightEffectAvailability, CapabilityBase};
use crate::{Color, Error};
use dbus_message_parser::Value;

/// Controls the reactive effect.
#[derive(Debug, Clone)]
pub struct StarlightEffect<'a> {
    pub(crate) base: CapabilityBase<'a>,
    pub(crate) available: StarlightEffectAvailability,
}

/// The effect's speed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Speed {
    /// The reaction stays for 500ms.
    Fast = 1,
    /// The reaction stays for 1000ms.
    Medium = 2,
    /// The reaction stays for 1500ms.
    Slow = 3,
}

/// The effect's mode.
pub enum Mode {
    /// Note: this is not supported on these devices:
    ///
    /// - Razer Blade Pro (Late 2016)
    /// - Razer Blade (Late 2016)
    /// - Razer Blade (QHD)
    /// - Razer BlackWidow Ultimate 2016
    /// - Razer BlackWidow X Ultimate
    /// - Razer Blade Pro (2017)
    /// - Razer Blade Pro FullHD (2017)
    /// - Razer Blade 15 (2018)
    /// - Razer Blade 15 (2018) Mercury
    Single(Color),
    /// Note: this is not supported on these devices:
    ///
    /// - Razer Blade Pro (Late 2016)
    /// - Razer Blade (Late 2016)
    /// - Razer Blade (QHD)
    /// - Razer BlackWidow Ultimate 2016
    /// - Razer BlackWidow X Ultimate
    /// - Razer Blade Pro (2017)
    /// - Razer Blade Pro FullHD (2017)
    /// - Razer Blade 15 (2018)
    /// - Razer Blade 15 (2018) Mercury
    /// - Razer Ornata
    Dual(Color, Color),
    /// Note: this is not supported on Razer Ornata
    Random,
}

impl StarlightEffect<'_> {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD_SINGLE: &'static str = "setStarlightSingle";
    pub(crate) const METHOD_DUAL: &'static str = "setStarlightDual";
    pub(crate) const METHOD_RANDOM: &'static str = "setStarlightRandom";

    /// Sets the breath effect.
    pub async fn set(&self, mode: Mode, speed: Speed) -> Result<(), Error> {
        match mode {
            Mode::Single((red, green, blue)) if self.available.single => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD_SINGLE,
                        vec![
                            Value::Byte(red),
                            Value::Byte(green),
                            Value::Byte(blue),
                            Value::Byte(speed as u8),
                        ],
                    )
                    .await
            }
            Mode::Dual(first, second) if self.available.dual => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD_DUAL,
                        vec![
                            Value::Byte(first.0),
                            Value::Byte(first.1),
                            Value::Byte(first.2),
                            Value::Byte(second.0),
                            Value::Byte(second.1),
                            Value::Byte(second.2),
                            Value::Byte(speed as u8),
                        ],
                    )
                    .await
            }
            Mode::Random if self.available.random => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD_RANDOM,
                        vec![Value::Byte(speed as u8)],
                    )
                    .await
            }
            _ => Err(Error::UnsupportedCapability),
        }
    }
}
