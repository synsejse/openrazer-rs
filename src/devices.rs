//! Various Razer devices.

mod generic;
mod huntsman;

pub use generic::{GenericDevice, GenericMethods};
pub use huntsman::Huntsman;

use crate::capabilities::{
    AvailableCapabilities, BreathEffect, Brightness, CapabilityBase, CustomEffect, GameMode,
    KeyboardLayout, MacroEffect, MacroMode, MatrixDimensions, NoEffect, ReactiveEffect,
    RippleEffect, SpectrumEffect, StarlightEffect, StaticEffect, WaveEffect,
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

    fn capability_base(&self) -> CapabilityBase {
        self.get_generic_device().capability_base()
    }

    /// Controls the breath effect.
    pub fn breath_effect(&self) -> Option<BreathEffect> {
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

    /// Controls the custom effect.
    pub fn custom_effect(&self) -> Option<CustomEffect> {
        let available = self.capabilities().custom_effect;
        if available != Default::default() {
            Some(CustomEffect {
                base: self.capability_base(),
                available,
            })
        } else {
            None
        }
    }

    /// Controls the device's brightness.
    pub fn brightness(&self) -> Option<Brightness> {
        if self.capabilities().brightness {
            Some(Brightness(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's Game Mode.
    pub fn game_mode(&self) -> Option<GameMode> {
        if self.capabilities().game_mode {
            Some(GameMode(self.capability_base()))
        } else {
            None
        }
    }

    /// Gets the device's keyboard layout.
    pub fn keyboard_layout(&self) -> Option<KeyboardLayout> {
        if self.capabilities().keyboard_layout {
            Some(KeyboardLayout(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's macro effect.
    pub fn macro_effect(&self) -> Option<MacroEffect> {
        if self.capabilities().macro_effect {
            Some(MacroEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the device's Macro Mode.
    pub fn macro_mode(&self) -> Option<MacroMode> {
        if self.capabilities().macro_mode {
            Some(MacroMode(self.capability_base()))
        } else {
            None
        }
    }

    /// Gets the device's matrix dimensions.
    pub fn matrix_dimensions(&self) -> Option<MatrixDimensions> {
        if self.capabilities().matrix_dimensions {
            Some(MatrixDimensions(self.capability_base()))
        } else {
            None
        }
    }

    /// Disables any lighting effect.
    pub fn no_effect(&self) -> Option<NoEffect> {
        if self.capabilities().no_effect {
            Some(NoEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the reactive effect.
    pub fn reactive_effect(&self) -> Option<ReactiveEffect> {
        if self.capabilities().reactive_effect {
            Some(ReactiveEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the ripple effect.
    pub fn ripple_effect(&self) -> Option<RippleEffect> {
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
    pub fn spectrum_effect(&self) -> Option<SpectrumEffect> {
        if self.capabilities().spectrum_effect {
            Some(SpectrumEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Sets the starlight effect.
    pub fn starlight_effect(&self) -> Option<StarlightEffect> {
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
    pub fn static_effect(&self) -> Option<StaticEffect> {
        if self.capabilities().static_effect {
            Some(StaticEffect(self.capability_base()))
        } else {
            None
        }
    }

    /// Controls the wave effect.
    pub fn wave_effect(&self) -> Option<WaveEffect> {
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
