//! Types for controlling a device's macro effect.

use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

/// Macro LED's mode.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Mode {
    Static = 0,
    Blink = 1,
}

/// Controls a device's macro effect.
#[derive(Debug, Clone)]
pub struct MacroEffect<'a>(pub(crate) CapabilityBase<'a>);

impl<'a> MacroEffect<'a> {
    pub(crate) const INTERFACE: &'static str = "razer.device.led.macromode";
    pub(crate) const GET_METHOD: &'static str = "getMacroEffect";
    const SET_METHOD: &'static str = "setMacroEffect";

    /// Gets the current macro effect.
    pub async fn get(&self) -> Result<Mode, Error> {
        self.0
            .call_get_method(Self::INTERFACE, Self::GET_METHOD)
            .await
            .and_then(|response| match response {
                Value::Int32(0) => Ok(Mode::Static),
                Value::Int32(1) => Ok(Mode::Blink),
                _ => Err(Error::InvalidResponse),
            })
    }

    /// Sets a macro effect.
    pub async fn set(&self, mode: Mode) -> Result<(), Error> {
        self.0
            .call_set_method(
                Self::INTERFACE,
                Self::SET_METHOD,
                vec![Value::Int32(mode as i32)],
            )
            .await
    }

    /// Sets a static macro effect.
    pub async fn set_static(&self) -> Result<(), Error> {
        self.set(Mode::Static).await
    }

    /// Sets a blink macro effect.
    pub async fn set_blink(&self) -> Result<(), Error> {
        self.set(Mode::Blink).await
    }
}
