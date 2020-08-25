use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

/// Controls a device's Game Mode.
#[derive(Debug, Clone)]
pub struct GameMode<'a>(pub(crate) CapabilityBase<'a>);

impl<'a> GameMode<'a> {
    pub(crate) const INTERFACE: &'static str = "razer.device.led.gamemode";
    pub(crate) const GET_METHOD: &'static str = "getGameMode";
    const SET_METHOD: &'static str = "setGameMode";

    /// Checks if Game Mode is enabled.
    pub async fn get(&self) -> Result<bool, Error> {
        self.0
            .call_get_method(Self::INTERFACE, Self::GET_METHOD)
            .await
            .and_then(|response| match response {
                Value::Boolean(is_enabled) => Ok(is_enabled),
                _ => Err(Error::InvalidResponse),
            })
    }

    /// Sets Game Mode status.
    pub async fn set(&self, is_enabled: bool) -> Result<(), Error> {
        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::SET_METHOD,
                vec![Value::Boolean(is_enabled)],
            )
            .await
    }

    /// Enables Game Mode.
    pub async fn enable(&self) -> Result<(), Error> {
        self.set(true).await
    }

    /// Disables Game Mode.
    pub async fn disable(&self) -> Result<(), Error> {
        self.set(false).await
    }
}
