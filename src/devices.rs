//! Various Razer devices.

mod generic;
mod huntsman;

pub use generic::{GenericDevice, GenericMethods};
pub use huntsman::Huntsman;

use crate::{
    capabilities::{
        Brightness, CapabilityBase, GameMode, KeyboardLayout, MacroEffect, MacroMode, WaveEffect,
    },
    Error,
};

/// A Razer device, possibly a specific one.
///
/// This enum represents some Razer device. It has methods for each capability
/// (e.g. brightness, effects and keyboard layout), but these methods check that
/// the capability is present, because they operate on an unkonwn device.
/// For read-and-write capabilities (e.g. brightness), a special struct is
/// returned if the capability is present and `None` otherwise. For read-only
/// or write-only capabilities (e.g. keyboard layout or effects), these methods
/// return `Err(Error::UnsupportedCapability)` if the capability isn't
/// present, otherwise the methods do their job (e.g. read the keyboard layout
/// or set an effect).
///
/// This crate also has types for specific devices, e.g. Razer Hunstman. If the
/// underlying device is known to this crate, it will construct a variant for
/// this speicifc device, e.g. `Device::Huntsman` instead of `Device::Other`
/// for yet unknown devices. You can match on this enum and use a type-safer
/// API that knows the device's capabilities, e.g. trying to set DPI for
/// a keyboard will result in a compile-time error.
#[derive(Debug, Clone)]
pub enum Device {
    /// A Razer Huntsman keyboard.
    Huntsman(Huntsman),
    /// A device unknown to this crate.
    Other(GenericDevice),
}

impl Device {
    fn has_capability(&self, interface: &'static str, method: &'static str) -> bool {
        self.get_generic_device()
            .available_features
            .get(interface)
            .map_or(false, |methods| methods.iter().any(|x| x == method))
    }

    fn get_capability<'a, T: 'a>(
        &'a self,
        construct: impl FnOnce(CapabilityBase<'a>) -> T,
        interface: &'static str,
        method: &'static str,
    ) -> Option<T> {
        if self.has_capability(interface, method) {
            Some(construct(self.get_generic_device().capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's brightness.
    pub fn brightness(&self) -> Option<Brightness<'_>> {
        self.get_capability(Brightness, Brightness::INTERFACE, Brightness::GET_METHOD)
    }

    /// Controls the device's Game Mode.
    pub fn game_mode(&self) -> Option<GameMode<'_>> {
        self.get_capability(GameMode, GameMode::INTERFACE, GameMode::GET_METHOD)
    }

    /// Gets the device's keyboard layout.
    pub async fn keyboard_layout(&self) -> Result<String, Error> {
        if self.has_capability(KeyboardLayout::INTERFACE, KeyboardLayout::METHOD) {
            KeyboardLayout::get(self.get_generic_device().capability_base()).await
        } else {
            Err(Error::UnsupportedCapability)
        }
    }

    /// Controls the device's macro effect.
    pub fn macro_effect(&self) -> Option<MacroEffect<'_>> {
        self.get_capability(MacroEffect, MacroEffect::INTERFACE, MacroEffect::GET_METHOD)
    }

    /// Controls the device's Macro Mode.
    pub fn macro_mode(&self) -> Option<MacroMode<'_>> {
        self.get_capability(MacroMode, MacroMode::INTERFACE, MacroMode::GET_METHOD)
    }

    /// Controls the wave effect..
    pub fn wave_effect(&self) -> Option<WaveEffect<'_>> {
        self.get_capability(WaveEffect, WaveEffect::INTERFACE, WaveEffect::METHOD)
    }
}

impl GenericMethods for Device {
    fn get_generic_device(&self) -> &GenericDevice {
        match self {
            Device::Huntsman(Huntsman(device)) => device,
            Device::Other(device) => device,
        }
    }
}
