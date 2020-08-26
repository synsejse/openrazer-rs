use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

/// Controls a device's brightness.
#[derive(Debug, Clone)]
pub struct Brightness(pub(crate) CapabilityBase);

impl Brightness {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.brightness";
    pub(crate) const GET_METHOD: &'static str = "getBrightness";
    const SET_METHOD: &'static str = "setBrightness";

    /// Gets the device's lighting brightness.
    pub async fn get(&self) -> Result<f64, Error> {
        self.0
            .call_get_method(Self::INTERFACE, Self::GET_METHOD)
            .await
            .and_then(|response| match response {
                Value::Double(brightness) => Ok(brightness),
                _ => Err(Error::InvalidResponse),
            })
    }

    /// Sets the device's lighting brightness.
    ///
    /// # Panics
    ///
    /// Panics if `brightness` is not in range `0.0..=100.0`.
    pub async fn set(&self, brightness: f64) -> Result<(), Error> {
        assert!(0.0 <= brightness && brightness <= 100.0);

        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::SET_METHOD,
                vec![Value::Double(brightness)],
            )
            .await
    }

    /// Disables lighting, i.e. sets the brightness to `0.0`.
    pub async fn turn_off(&self) -> Result<(), Error> {
        self.set(0.0).await
    }

    /// Sets maximum brightness, `100.0`.
    pub async fn full(&self) -> Result<(), Error> {
        self.set(100.0).await
    }
}
