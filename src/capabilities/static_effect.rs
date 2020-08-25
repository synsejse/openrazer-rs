use super::CapabilityBase;
use crate::{Color, Error};
use dbus_message_parser::Value;

/// Controls the static effect.
#[derive(Debug, Clone)]
pub struct StaticEffect<'a>(pub(crate) CapabilityBase<'a>);

impl StaticEffect<'_> {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD: &'static str = "setStatic";

    /// Sets the static effect.
    pub async fn set(&self, (red, green, blue): Color) -> Result<(), Error> {
        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::METHOD,
                vec![Value::Byte(red), Value::Byte(green), Value::Byte(blue)],
            )
            .await
    }
}
