use super::devices_message;
use crate::Error;
use dbus_async::DBus;
use dbus_message_parser::Value;

/// Controls whether devices' lighting is disabled on lockscreen.
pub struct TurnOffOnScreensaver {
    pub(crate) dbus: DBus,
}

impl TurnOffOnScreensaver {
    /// Returns whether this setting is enabled.
    pub async fn get(&self) -> Result<bool, Error> {
        let response = self
            .dbus
            .call(devices_message("getOffOnScreensaver"))
            .await?;

        match response.get_body() {
            [Value::Boolean(is_enabled)] => Ok(*is_enabled),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Controls this setting.
    pub async fn set(&self, is_enabled: bool) -> Result<(), Error> {
        let mut message = devices_message("enableTurnOffOnScreensaver");
        message.add_value(Value::Boolean(is_enabled));

        self.dbus.call(message).await?;
        Ok(())
    }

    /// Enables this setting.
    pub async fn enable(&self) -> Result<(), Error> {
        self.set(true).await
    }

    /// Disables this setting.
    pub async fn disable(&self) -> Result<(), Error> {
        self.set(false).await
    }
}
