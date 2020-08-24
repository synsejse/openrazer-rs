use dbus_async::DBus;
use dbus_message_parser::{Message, Value};
use std::{collections::HashMap, io};

use crate::Error;

const DESTINATION: &str = "org.razer";
const PATH: &str = "/org/razer";
const DAEMON_INTERFACE: &str = "razer.daemon";
const DEVICES_INTERFACE: &str = "razer.devices";

/// Manages Razer devices.
#[derive(Debug)]
pub struct DeviceManager {
    dbus: DBus,
}

fn daemon_message(method: &'static str) -> Message {
    Message::method_call(DESTINATION, PATH, DAEMON_INTERFACE, method)
}

fn devices_message(method: &'static str) -> Message {
    Message::method_call(DESTINATION, PATH, DEVICES_INTERFACE, method)
}

impl DeviceManager {
    /// Constructs a `DeviceManager`.
    pub async fn new() -> io::Result<Self> {
        let (dbus, _) = DBus::session(true).await?;

        Ok(Self { dbus })
    }

    /// Fetches the daemon's version.
    pub async fn get_daemon_version(&self) -> Result<String, Error> {
        let (_, body) = self.dbus.call(daemon_message("version")).await?.split();

        match body.into_iter().next() {
            Some(Value::String(version)) => Ok(version),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Stops the daemon.
    pub async fn stop_daemon(&self) -> Result<(), Error> {
        self.dbus.call(daemon_message("stop")).await?;
        Ok(())
    }

    /// Fetches supported devices.
    pub async fn get_supported_devices(&self) -> Result<HashMap<String, [u16; 2]>, Error> {
        let response = self.dbus.call(devices_message("supportedDevices")).await?;

        match response.get_body() {
            [Value::String(json)] => serde_json::from_str(json).or(Err(Error::InvalidResponse)),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Fetches information if devices' lighting is disabled on lockscreen.
    pub async fn get_turn_off_on_screensaver(&self) -> Result<bool, Error> {
        let response = self
            .dbus
            .call(devices_message("getOffOnScreensaver"))
            .await?;

        match response.get_body() {
            [Value::Boolean(is_enabled)] => Ok(*is_enabled),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets whether devices' lighting is disabled on lockscreen.
    pub async fn set_turn_off_on_screensaver(&self, is_enabled: bool) -> Result<(), Error> {
        let mut message = devices_message("enableTurnOffOnScreensaver");
        message.add_value(Value::Boolean(is_enabled));

        self.dbus.call(message).await?;
        Ok(())
    }

    /// Fetches information if applied effects are synchronized across all devices.
    pub async fn get_sync_effect(&self) -> Result<bool, Error> {
        let response = self.dbus.call(devices_message("getSyncEffects")).await?;

        match response.get_body() {
            [Value::Boolean(should_sync)] => Ok(*should_sync),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Sets whether applied effects are synchronized across all devices.
    pub async fn set_sync_effect(&self, should_sync: bool) -> Result<(), Error> {
        let mut message = devices_message("syncEffects");
        message.add_value(Value::Boolean(should_sync));

        self.dbus.call(message).await?;
        Ok(())
    }
}
