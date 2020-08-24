use crate::{
    devices::{Device, Keyboard},
    Error,
};
use dbus_async::DBus;
use dbus_message_parser::{Message, Value};
use futures::future::BoxFuture;
use std::collections::HashMap;
use xmltree::Element;

pub(crate) const DEVICE_INTERFACE: &str = "razer.device.misc";
const BRIGHTNESS_INTERFACE: &str = "razer.device.lighting.brightness";
const INTROSPECT_INTERFACE: &str = "org.freedesktop.DBus.Introspectable";

type AvailableFeatures = HashMap<String, Vec<String>>;

/// Any Razer device, without device-specific methods.
#[derive(Debug, Clone)]
pub struct GenericDevice {
    pub(crate) dbus: DBus,
    pub(crate) serial: String,
    pub(crate) path: String,
    pub(crate) available_features: AvailableFeatures,
}

/// Methods available on any device.
pub trait GenericMethods {
    /// Gets the underlying `GenericDevice`.
    fn get_generic_device(&self) -> &GenericDevice;

    /// Gets the device's serial.
    fn get_serial(&self) -> &str {
        &self.get_generic_device().serial
    }

    /// Gets the device's name.
    fn get_name(&self) -> BoxFuture<'_, Result<String, Error>> {
        let device = self.get_generic_device();
        Box::pin(async move {
            let (_, body) = device
                .dbus
                .call(device.message(DEVICE_INTERFACE, "getDeviceName"))
                .await?
                .split();

            match body.into_iter().next() {
                Some(Value::String(name)) => Ok(name),
                _ => Err(Error::InvalidResponse),
            }
        })
    }

    /// Gets the device's firmware version.
    fn get_firmware_version(&self) -> BoxFuture<'_, Result<String, Error>> {
        let device = self.get_generic_device();
        Box::pin(async move {
            let (_, body) = device
                .dbus
                .call(device.message(DEVICE_INTERFACE, "getFirmware"))
                .await?
                .split();

            match body.into_iter().next() {
                Some(Value::String(version)) => Ok(version),
                _ => Err(Error::InvalidResponse),
            }
        })
    }

    /// Gets the driver version for the device.
    fn get_driver_version(&self) -> BoxFuture<'_, Result<String, Error>> {
        let device = self.get_generic_device();
        Box::pin(async move {
            let (_, body) = device
                .dbus
                .call(device.message(DEVICE_INTERFACE, "getDriverVersion"))
                .await?
                .split();

            match body.into_iter().next() {
                Some(Value::String(version)) => Ok(version),
                _ => Err(Error::InvalidResponse),
            }
        })
    }

    /// Gets the device's lighting brightness.
    fn get_brightness(&self) -> BoxFuture<'_, Result<f64, Error>> {
        let device = self.get_generic_device();
        Box::pin(async move {
            let response = device
                .dbus
                .call(device.message(BRIGHTNESS_INTERFACE, "getBrightness"))
                .await?;

            match response.get_body() {
                [Value::Double(brightness)] => Ok(*brightness),
                _ => Err(Error::InvalidResponse),
            }
        })
    }

    /// Sets the device's lighting brightness.
    ///
    /// # Panics
    ///
    /// Panics if `brightness` is not in range `0.0..=100.0`.
    fn set_brightness(&self, brightness: f64) -> BoxFuture<'_, Result<(), Error>> {
        assert!(0.0 <= brightness && brightness <= 100.0);

        let device = self.get_generic_device();
        Box::pin(async move {
            let mut message = device.message(BRIGHTNESS_INTERFACE, "setBrightness");
            message.add_value(Value::Double(brightness));

            device.dbus.call(message).await?;
            Ok(())
        })
    }

    /// Checks if the device has dedicated macro keys.
    fn has_dedicated_macro_keys(&self) -> BoxFuture<'_, Result<bool, Error>> {
        let device = self.get_generic_device();
        Box::pin(async move {
            let response = device
                .dbus
                .call(device.message(DEVICE_INTERFACE, "hasDedicatedMacroKeys"))
                .await?;

            match response.get_body() {
                [Value::Boolean(has_keys)] => Ok(*has_keys),
                _ => Err(Error::InvalidResponse),
            }
        })
    }

    /// Gets Razer URLs.
    fn get_razer_urls(&self) -> BoxFuture<'_, Result<HashMap<String, String>, Error>> {
        let device = self.get_generic_device();
        Box::pin(async move {
            let response = device
                .dbus
                .call(device.message(DEVICE_INTERFACE, "getRazerUrls"))
                .await?;

            match response.get_body() {
                [Value::String(json)] => serde_json::from_str(json).or(Err(Error::InvalidResponse)),
                _ => Err(Error::InvalidResponse),
            }
        })
    }
}

impl GenericDevice {
    pub(crate) async fn new(dbus: DBus, serial: String) -> Result<Self, Error> {
        let path = format!("/org/razer/device/{}", &serial);
        let available_features = get_available_features(&dbus, &path).await?;
        Ok(Self {
            dbus,
            serial,
            path,
            available_features,
        })
    }

    pub(crate) fn message(&self, interface: &'static str, method: &'static str) -> Message {
        Message::method_call(crate::DESTINATION, self.path.as_str(), interface, method)
    }

    pub(crate) async fn into_device(self) -> Result<Device, Error> {
        let response = self
            .dbus
            .call(self.message(DEVICE_INTERFACE, "getDeviceType"))
            .await?;

        let device_type = match response.get_body() {
            [Value::String(device_type)] => device_type.as_str(),
            _ => return Err(Error::InvalidResponse),
        };

        Ok(match device_type {
            "keyboard" | "keypad" => Device::Keyboard(Keyboard::new(self)),
            // todo: "mouse"
            // todo: "mousemat"
            _ => Device::Other(self),
        })
    }
}

async fn get_available_features(dbus: &DBus, path: &str) -> Result<AvailableFeatures, Error> {
    let message =
        Message::method_call(crate::DESTINATION, path, INTROSPECT_INTERFACE, "Introspect");
    let response = dbus.call(message).await?;
    let xml = match response.get_body() {
        [Value::String(xml)] => xml,
        _ => return Err(Error::InvalidResponse),
    };
    let mut node = Element::parse(xml.as_bytes()).or(Err(Error::InvalidResponse))?;

    let mut features = HashMap::new();

    while let Some(mut interface) = node.take_child("interface") {
        let interface_name = interface
            .attributes
            .get("name")
            .ok_or(Error::InvalidResponse)?
            .to_owned();

        if interface_name == INTROSPECT_INTERFACE {
            continue;
        }

        while let Some(method) = interface.take_child("method") {
            let method_name = method
                .attributes
                .get("name")
                .ok_or(Error::InvalidResponse)?
                .to_owned();

            features
                .entry(interface_name.clone())
                .or_insert_with(Vec::new)
                .push(method_name);
        }
    }

    Ok(features)
}

impl GenericMethods for GenericDevice {
    fn get_generic_device(&self) -> &GenericDevice {
        self
    }
}
