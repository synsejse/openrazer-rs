//! Various Razer devices.

mod generic;
mod huntsman;

pub use generic::{GenericDevice, GenericMethods};
pub use huntsman::Huntsman;

use crate::capabilities::{
    AvailableCapabilities, BreathEffect, Brightness, CapabilityBase, GameMode, KeyboardLayout,
    MacroEffect, MacroMode, NoEffect, ReactiveEffect, RippleEffect, SpectrumEffect,
    StarlightEffect, StaticEffect, WaveEffect,
};

/// A Razer device, possibly a specific one.
///
/// This enum represents some Razer device. It has methods for each capability
/// (e.g. brightness, effects and keyboard layout), but these methods check that
/// the capability is present, because they operate on an unkonwn device.
/// If it is, then a special struct, which controls the capatibily, is returned.
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
    fn capabilities(&self) -> AvailableCapabilities {
        self.get_generic_device().available
    }

    fn capability_base(&self) -> CapabilityBase<'_> {
        self.get_generic_device().capability_base()
    }

    /// Controls the breath effect.
    pub fn breath_effect(&self) -> Option<BreathEffect<'_>> {
        let available = self.capabilities().breath_effect;
        if available.single || available.dual || available.triple || available.random {
            Some(BreathEffect {
                base: self.capability_base(),
                available,
            })
        } else {
            None
        }
    }

    /// Controls the device's brightness.
    pub fn brightness(&self) -> Option<Brightness<'_>> {
        if self.capabilities().brightness {
            Some(Brightness(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's Game Mode.
    pub fn game_mode(&self) -> Option<GameMode<'_>> {
        if self.capabilities().game_mode {
            Some(GameMode(self.capability_base()))
        } else {
            None
        }
    }

    /// Gets the device's keyboard layout.
    pub fn keyboard_layout(&self) -> Option<KeyboardLayout<'_>> {
        if self.capabilities().keyboard_layout {
            Some(KeyboardLayout(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's macro effect.
    pub fn macro_effect(&self) -> Option<MacroEffect<'_>> {
        if self.capabilities().macro_effect {
            Some(MacroEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's Macro Mode.
    pub fn macro_mode(&self) -> Option<MacroMode<'_>> {
        if self.capabilities().macro_mode {
            Some(MacroMode(self.capability_base()))
        } else {
            None
        }
    }

    /// Disables any lighting effect.
    pub fn no_effect(&self) -> Option<NoEffect<'_>> {
        if self.capabilities().no_effect {
            Some(NoEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the reactive effect.
    pub fn reactive_effect(&self) -> Option<ReactiveEffect<'_>> {
        if self.capabilities().reactive_effect {
            Some(ReactiveEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the ripple effect.
    pub fn ripple_effect(&self) -> Option<RippleEffect<'_>> {
        let available = self.capabilities().ripple_effect;
        if available.single || available.random {
            Some(RippleEffect {
                base: self.capability_base(),
                available,
            })
        } else {
            None
        }
    }

    /// Sets the spectrum effect.
    pub fn spectrum_effect(&self) -> Option<SpectrumEffect<'_>> {
        if self.capabilities().spectrum_effect {
            Some(SpectrumEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Sets the starlight effect.
    pub fn starlight_effect(&self) -> Option<StarlightEffect<'_>> {
        let available = self.capabilities().starlight_effect;

        if available.single || available.random || available.dual {
            Some(StarlightEffect {
                base: self.capability_base(),
                available,
            })
        } else {
            None
        }
    }

    /// Controls the static effect.
    pub fn static_effect(&self) -> Option<StaticEffect<'_>> {
        if self.capabilities().static_effect {
            Some(StaticEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the wave effect.
    pub fn wave_effect(&self) -> Option<WaveEffect<'_>> {
        if self.capabilities().wave_effect {
            Some(WaveEffect(self.capability_base()))
        } else {
            None
        }
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
