use crate::{
    devices::{Device, GenericDevice},
    Error,
};
use dbus_async::DBus;
use dbus_message_parser::{Message, Value};
use std::{collections::HashMap, io};

mod sync_effects;
mod turn_off_on_screensaver;

pub use sync_effects::SyncEffects;
pub use turn_off_on_screensaver::TurnOffOnScreensaver;

const PATH: &str = "/org/razer";
const DAEMON_INTERFACE: &str = "razer.daemon";
const DEVICES_INTERFACE: &str = "razer.devices";

/// Manages Razer devices.
#[derive(Debug)]
pub struct DeviceManager {
    dbus: DBus,
}

fn daemon_message(method: &'static str) -> Message {
    Message::method_call(crate::DESTINATION, PATH, DAEMON_INTERFACE, method)
}

fn devices_message(method: &'static str) -> Message {
    Message::method_call(crate::DESTINATION, PATH, DEVICES_INTERFACE, method)
}

impl DeviceManager {
    /// Constructs a `DeviceManager`.
    pub async fn new() -> io::Result<Self> {
        let (dbus, _) = DBus::session(true).await?;

        Ok(Self { dbus })
    }

    /// Fetches the daemon's version.
    pub async fn daemon_version(&self) -> Result<String, Error> {
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
    pub async fn supported_devices(&self) -> Result<HashMap<String, [u16; 2]>, Error> {
        let response = self.dbus.call(devices_message("supportedDevices")).await?;

        match response.get_body() {
            [Value::String(json)] => serde_json::from_str(json).or(Err(Error::InvalidResponse)),
            _ => Err(Error::InvalidResponse),
        }
    }

    /// Fetches connected devices.
    pub async fn devices(&self) -> Result<Vec<Device>, Error> {
        let (_, body) = self.dbus.call(devices_message("getDevices")).await?.split();

        let serials = match body.into_iter().next() {
            Some(Value::Array(serials, _)) => serials,
            _ => return Err(Error::InvalidResponse),
        };

        let mut devices = Vec::new();

        for serial in serials {
            let serial = match serial {
                Value::String(serial) => serial,
                _ => return Err(Error::InvalidResponse),
            };

            let generic = GenericDevice::new(self.dbus.clone(), serial).await?;
            devices.push(generic.into_device().await?);
        }

        Ok(devices)
    }

    /// Controls whether devices' lighting is disabled on lockscreen.
    pub fn turn_off_on_screensaver(&self) -> TurnOffOnScreensaver {
        TurnOffOnScreensaver {
            dbus: self.dbus.clone(),
        }
    }

    /// Conrols whether applied effects are synchronized across all devices.
    pub fn sync_effects(&self) -> SyncEffects {
        SyncEffects {
            dbus: self.dbus.clone(),
        }
    }
}
