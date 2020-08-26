//! Types for controlling the ripple effect.

use super::{available::RippleEffectAvailability, CapabilityBase};
use crate::{Color, Error};
use dbus_message_parser::Value;

/// The ripple effect's mode.
pub enum Mode {
    Single(Color),
    /// Note: Razer BlackWidow Ultimate 2016 and Razer BlackWidow X Ultimate
    /// don't support this mode.
    Random,
}

/// Controls the ripple effect.
#[derive(Debug, Clone)]
pub struct RippleEffect {
    pub(crate) base: CapabilityBase,
    pub(crate) availability: RippleEffectAvailability,
}

impl RippleEffect {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.custom";
    pub(crate) const METHOD: &'static str = "setRipple";
    pub(crate) const METHOD_RANDOM: &'static str = "setRippleRandomColour";

    /// Sets the ripple effect.
    pub async fn set(&self, mode: Mode, refresh_rate: f64) -> Result<(), Error> {
        match mode {
            Mode::Single((red, green, blue)) if self.availability.single => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD,
                        vec![
                            Value::Byte(red),
                            Value::Byte(green),
                            Value::Byte(blue),
                            Value::Double(refresh_rate),
                        ],
                    )
                    .await
            }
            Mode::Random if self.availability.random => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD_RANDOM,
                        vec![Value::Double(refresh_rate)],
                    )
                    .await
            }
            _ => Err(Error::UnsupportedCapability),
        }
    }

    /// Sets the ripple effect with `Mode::Single`
    pub async fn single(&self, color: Color, refresh_rate: f64) -> Result<(), Error> {
        self.set(Mode::Single(color), refresh_rate).await
    }

    /// Sets the ripple effect with `Mode::Random`
    pub async fn random(&self, refresh_rate: f64) -> Result<(), Error> {
        self.set(Mode::Random, refresh_rate).await
    }
}
