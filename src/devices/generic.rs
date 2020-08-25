use super::Huntsman;
use crate::{
    capabilities::{
        AvailableCapabilities, BreathEffect, Brightness, CapabilityBase, GameMode, KeyboardLayout,
        MacroEffect, MacroMode, NoEffect, ReactiveEffect, RippleEffect, SpectrumEffect,
        StarlightEffect, StaticEffect, WaveEffect,
    },
    devices::Device,
    Error,
};
use dbus_async::DBus;
use dbus_message_parser::{Message, Value};
use futures::future::BoxFuture;
use std::collections::HashMap;
use xmltree::Element;

pub(crate) const DEVICE_INTERFACE: &str = "razer.device.misc";
const INTROSPECT_INTERFACE: &str = "org.freedesktop.DBus.Introspectable";

/// Any Razer device, without device-specific methods.
#[derive(Debug, Clone)]
pub struct GenericDevice {
    pub(crate) dbus: DBus,
    pub(crate) serial: String,
    pub(crate) path: String,
    pub(crate) available: AvailableCapabilities,
}

/// Methods available on any device.
pub trait GenericMethods {
    /// Gets the underlying `GenericDevice`.
    fn get_generic_device(&self) -> &GenericDevice;

    /// Gets the device's serial.
    fn serial(&self) -> &str {
        &self.get_generic_device().serial
    }

    /// Gets available capabilities.
    fn capabilities(&self) -> AvailableCapabilities {
        self.get_generic_device().available
    }

    /// Gets the device's name.
    fn name(&self) -> BoxFuture<'_, Result<String, Error>> {
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
    fn firmware_version(&self) -> BoxFuture<'_, Result<String, Error>> {
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
    fn driver_version(&self) -> BoxFuture<'_, Result<String, Error>> {
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
    fn razer_urls(&self) -> BoxFuture<'_, Result<HashMap<String, String>, Error>> {
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
        let available_capabilities = get_available_capabilities(&dbus, &path).await?;
        Ok(Self {
            dbus,
            serial,
            path,
            available: available_capabilities,
        })
    }

    pub(crate) fn message(&self, interface: &'static str, method: &'static str) -> Message {
        Message::method_call(crate::DESTINATION, self.path.as_str(), interface, method)
    }

    pub(crate) fn capability_base(&self) -> CapabilityBase<'_> {
        CapabilityBase::new(self.dbus.clone(), &self.path)
    }

    pub(crate) async fn into_device(self) -> Result<Device, Error> {
        Ok(match self.name().await?.as_str() {
            Huntsman::NAME => Device::Huntsman(Huntsman(self)),
            _ => Device::Other(self),
        })
    }
}

async fn get_available_capabilities(
    dbus: &DBus,
    path: &str,
) -> Result<AvailableCapabilities, Error> {
    let message =
        Message::method_call(crate::DESTINATION, path, INTROSPECT_INTERFACE, "Introspect");
    let response = dbus.call(message).await?;
    let xml = match response.get_body() {
        [Value::String(xml)] => xml,
        _ => return Err(Error::InvalidResponse),
    };
    let mut node = Element::parse(xml.as_bytes()).or(Err(Error::InvalidResponse))?;

    let mut capabilities = AvailableCapabilities::default();

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
                .ok_or(Error::InvalidResponse)?;

            match (interface_name.as_str(), method_name.as_str()) {
                (BreathEffect::INTERFACE, BreathEffect::METHOD_SINGLE) => {
                    capabilities.breath_effect.single = true
                }
                (BreathEffect::INTERFACE, BreathEffect::METHOD_DUAL) => {
                    capabilities.breath_effect.dual = true
                }
                (BreathEffect::INTERFACE, BreathEffect::METHOD_TRIPLE) => {
                    capabilities.breath_effect.triple = true
                }
                (BreathEffect::INTERFACE, BreathEffect::METHOD_RANDOM) => {
                    capabilities.breath_effect.random = true
                }
                (Brightness::INTERFACE, Brightness::GET_METHOD) => capabilities.brightness = true,
                (GameMode::INTERFACE, GameMode::GET_METHOD) => capabilities.game_mode = true,
                (KeyboardLayout::INTERFACE, KeyboardLayout::METHOD) => {
                    capabilities.keyboard_layout = true
                }
                (MacroEffect::INTERFACE, MacroEffect::GET_METHOD) => {
                    capabilities.macro_effect = true
                }
                (MacroMode::INTERFACE, MacroMode::GET_METHOD) => capabilities.macro_mode = true,
                (NoEffect::INTERFACE, NoEffect::METHOD) => capabilities.no_effect = true,
                (ReactiveEffect::INTERFACE, ReactiveEffect::METHOD) => {
                    capabilities.reactive_effect = true
                }
                (RippleEffect::INTERFACE, RippleEffect::METHOD) => {
                    capabilities.ripple_effect.single = true
                }
                (RippleEffect::INTERFACE, RippleEffect::METHOD_RANDOM) => {
                    capabilities.ripple_effect.random = true
                }
                (SpectrumEffect::INTERFACE, SpectrumEffect::METHOD) => {
                    capabilities.spectrum_effect = true
                }
                (StarlightEffect::INTERFACE, StarlightEffect::METHOD_SINGLE) => {
                    capabilities.starlight_effect.single = true
                }
                (StarlightEffect::INTERFACE, StarlightEffect::METHOD_DUAL) => {
                    capabilities.starlight_effect.dual = true
                }
                (StarlightEffect::INTERFACE, StarlightEffect::METHOD_RANDOM) => {
                    capabilities.starlight_effect.random = true
                }
                (StaticEffect::INTERFACE, StaticEffect::METHOD) => {
                    capabilities.static_effect = true
                }
                (WaveEffect::INTERFACE, WaveEffect::METHOD) => capabilities.wave_effect = true,
                _ => (),
            }
        }
    }

    Ok(capabilities)
}

impl GenericMethods for GenericDevice {
    fn get_generic_device(&self) -> &GenericDevice {
        self
    }
}
