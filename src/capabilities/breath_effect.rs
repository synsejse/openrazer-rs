use super::{available::BreathEffectAvailability, CapabilityBase};
use crate::{Color, Error};
use dbus_message_parser::Value;

/// The breath effect's mode.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Mode {
    Single(Color),
    Dual(Color, Color),
    /// Note: this mode is supported only on Razer Kraken 7.1 V2.
    Triple(Color, Color, Color),
    Random,
}

/// Controls the breath effect.
#[derive(Debug, Clone)]
pub struct BreathEffect {
    pub(crate) base: CapabilityBase,
    pub(crate) availability: BreathEffectAvailability,
}

impl BreathEffect {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD_SINGLE: &'static str = "setBreathSingle";
    pub(crate) const METHOD_DUAL: &'static str = "setBreathDual";
    pub(crate) const METHOD_TRIPLE: &'static str = "setBreathTriple";
    pub(crate) const METHOD_RANDOM: &'static str = "setBreathRandom";

    /// Sets the breath effect.
    pub async fn set(&self, mode: Mode) -> Result<(), Error> {
        match mode {
            Mode::Single((red, green, blue)) if self.availability.single => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD_SINGLE,
                        vec![Value::Byte(red), Value::Byte(green), Value::Byte(blue)],
                    )
                    .await
            }
            Mode::Dual(first, second) if self.availability.dual => {
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
                        ],
                    )
                    .await
            }
            Mode::Triple(first, second, third) if self.availability.triple => {
                self.base
                    .call_set_method(
                        Self::INTERFACE,
                        Self::METHOD_TRIPLE,
                        vec![
                            Value::Byte(first.0),
                            Value::Byte(first.1),
                            Value::Byte(first.2),
                            Value::Byte(second.0),
                            Value::Byte(second.1),
                            Value::Byte(second.2),
                            Value::Byte(third.0),
                            Value::Byte(third.1),
                            Value::Byte(third.2),
                        ],
                    )
                    .await
            }
            Mode::Random if self.availability.random => {
                self.base
                    .call_set_method(Self::INTERFACE, Self::METHOD_RANDOM, Vec::new())
                    .await
            }
            _ => Err(Error::UnsupportedCapability),
        }
    }

    /// Sets the breath effect with `Mode::Single`.
    pub async fn single(&self, color: Color) -> Result<(), Error> {
        self.set(Mode::Single(color)).await
    }

    /// Sets the breath effect with `Mode::Dual`.
    pub async fn dual(&self, first: Color, second: Color) -> Result<(), Error> {
        self.set(Mode::Dual(first, second)).await
    }

    /// Sets the breath effect with `Mode::Triple`.
    ///
    /// Note: this mode is supported only on Razer Kraken 7.1 V2.
    pub async fn triple(&self, first: Color, second: Color, third: Color) -> Result<(), Error> {
        self.set(Mode::Triple(first, second, third)).await
    }

    /// Sets the breath effect with `Mode::Random`.
    pub async fn random(&self) -> Result<(), Error> {
        self.set(Mode::Random).await
    }
}
