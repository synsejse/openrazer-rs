use crate::{
    devices::generic::{GenericDevice, GenericMethods, DEVICE_INTERFACE},
    Error,
};
use dbus_message_parser::Value;

const GAME_MODE_INTERFACE: &str = "razer.device.led.gamemode";
const MACRO_MODE_INTERFACE: &str = "razer.device.led.macromode";
const PROFILE_LED_INTERFACE: &str = "razer.device.lighting.profile_led";

/// A Razer keyboard.
#[derive(Debug, Clone)]
pub struct Keyboard {
    base: GenericDevice,
}

/// Macro LED's mode.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum MacroEffect {
    Static = 0,
    Blink = 1,
}

impl Keyboard {
    pub(crate) fn new(base: GenericDevice) -> Self {
        Self { base }
    }
}

impl Keyboard {
    /// Fetches the keyboard's layout.
    pub async fn get_keyboard_layout(&self) -> Result<String, Error> {
        let (_, body) = self
            .base
            .dbus
            .call(self.base.message(DEVICE_INTERFACE, "getKeyboardLayout"))
            .await?
            .split();

        match body.into_iter().next() {
            Some(Value::String(name)) => Ok(name),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Fetches whether the game mode is enabled.
    pub async fn get_game_mode(&self) -> Result<bool, Error> {
        let response = self
            .base
            .dbus
            .call(self.base.message(GAME_MODE_INTERFACE, "getGameMode"))
            .await?;

        match response.get_body() {
            [Value::Boolean(is_enabled)] => Ok(*is_enabled),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets whether the game mode is enabled.
    pub async fn set_game_mode(&self, is_enabled: bool) -> Result<(), Error> {
        let mut message = self.base.message(GAME_MODE_INTERFACE, "setGameMode");
        message.add_value(Value::Boolean(is_enabled));

        self.base.dbus.call(message).await?;
        Ok(())
    }

    /// Fetches whether the macro mode is enabled.
    pub async fn get_macro_mode(&self) -> Result<bool, Error> {
        let response = self
            .base
            .dbus
            .call(self.base.message(MACRO_MODE_INTERFACE, "getMacroMode"))
            .await?;

        match response.get_body() {
            [Value::Boolean(is_enabled)] => Ok(*is_enabled),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets whether the macro mode is enabled.
    pub async fn set_macro_mode(&self, is_enabled: bool) -> Result<(), Error> {
        let mut message = self.base.message(MACRO_MODE_INTERFACE, "setMacroMode");
        message.add_value(Value::Boolean(is_enabled));

        self.base.dbus.call(message).await?;
        Ok(())
    }

    /// Fetches the macro effect.
    pub async fn get_macro_effect(&self) -> Result<MacroEffect, Error> {
        let response = self
            .base
            .dbus
            .call(self.base.message(MACRO_MODE_INTERFACE, "getMacroEffect"))
            .await?;

        match dbg!(response.get_body()) {
            [Value::Int32(0)] => Ok(MacroEffect::Static),
            [Value::Int32(1)] => Ok(MacroEffect::Blink),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets the macro effect.
    pub async fn set_macro_effect(&self, effect: MacroEffect) -> Result<(), Error> {
        let mut message = self.base.message(MACRO_MODE_INTERFACE, "setMacroEffect");
        message.add_value(Value::Int32(effect as i32));

        self.base.dbus.call(message).await?;
        Ok(())
    }

    /// Fetches red LED profile.
    pub async fn get_red_led_profile(&self) -> Result<bool, Error> {
        let response = self
            .base
            .dbus
            .call(self.base.message(PROFILE_LED_INTERFACE, "getRedLED"))
            .await?;

        match dbg!(response.get_body()) {
            [Value::Boolean(red_profile)] => Ok(*red_profile),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets red LED profile.
    pub async fn set_red_led_profile(&self, red_profile: bool) -> Result<(), Error> {
        let mut message = self.base.message(PROFILE_LED_INTERFACE, "setRedLED");
        message.add_value(Value::Boolean(red_profile));

        self.base.dbus.call(message).await?;
        Ok(())
    }

    /// Fetches green LED profile.
    pub async fn get_green_led_rofile(&self) -> Result<bool, Error> {
        let response = self
            .base
            .dbus
            .call(self.base.message(PROFILE_LED_INTERFACE, "getGreenLED"))
            .await?;

        match dbg!(response.get_body()) {
            [Value::Boolean(green_profile)] => Ok(*green_profile),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets green LED profile.
    pub async fn set_green_led_profile(&self, green_profile: bool) -> Result<(), Error> {
        let mut message = self.base.message(PROFILE_LED_INTERFACE, "setGreenLED");
        message.add_value(Value::Boolean(green_profile));

        self.base.dbus.call(message).await?;
        Ok(())
    }

    /// Fetches blue LED profile.
    pub async fn get_blue_led_rofile(&self) -> Result<bool, Error> {
        let response = self
            .base
            .dbus
            .call(self.base.message(PROFILE_LED_INTERFACE, "getBlueLED"))
            .await?;

        match dbg!(response.get_body()) {
            [Value::Boolean(blue_profile)] => Ok(*blue_profile),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets blue LED profile.
    pub async fn set_blue_led_profile(&self, blue_profile: bool) -> Result<(), Error> {
        let mut message = self.base.message(PROFILE_LED_INTERFACE, "setBlueLED");
        message.add_value(Value::Boolean(blue_profile));

        self.base.dbus.call(message).await?;
        Ok(())
    }
}

impl GenericMethods for Keyboard {
    fn get_generic_device(&self) -> &GenericDevice {
        &self.base
    }
}
