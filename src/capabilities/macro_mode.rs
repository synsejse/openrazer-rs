use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

/// Controls a device's Macro Mode.
#[derive(Debug, Clone)]
pub struct MacroMode(pub(crate) CapabilityBase);

impl MacroMode {
    pub(crate) const INTERFACE: &'static str = "razer.device.led.macromode";
    pub(crate) const GET_METHOD: &'static str = "getMacroMode";
    const SET_METHOD: &'static str = "setMacroMode";

    /// Checks if Macro Mode is enabled.
    pub async fn get(&self) -> Result<bool, Error> {
        self.0
            .call_get_method(Self::INTERFACE, Self::GET_METHOD)
            .await
            .and_then(|response| match response {
                Value::Boolean(is_enabled) => Ok(is_enabled),
                _ => Err(Error::InvalidResponse),
            })
    }

    /// Sets Macro Mode status.
    pub async fn set(&self, is_enabled: bool) -> Result<(), Error> {
        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::SET_METHOD,
                vec![Value::Boolean(is_enabled)],
            )
            .await
    }

    /// Enables Macro Mode.
    pub async fn enable(&self) -> Result<(), Error> {
        self.set(true).await
    }

    /// Disables Macro Mode.
    pub async fn disable(&self) -> Result<(), Error> {
        self.set(false).await
    }
}
