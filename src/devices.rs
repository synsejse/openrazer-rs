//! Various Razer devices.

mod generic;
mod huntsman;

pub use generic::{GenericDevice, GenericMethods};
pub use huntsman::Huntsman;

use crate::capabilities::{
    BreathEffect, Brightness, CapabilityBase, GameMode, KeyboardLayout, MacroEffect, MacroMode,
    NoEffect, ReactiveEffect, RippleEffect, SpectrumEffect, StarlightEffect, StaticEffect,
    WaveEffect,
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

    /// Controls the breath effect.
    pub fn breath_effect(&self) -> Option<BreathEffect<'_>> {
        if self.has_capability(BreathEffect::INTERFACE, BreathEffect::METHOD_SINGLE) {
            Some(BreathEffect {
                base: self.get_generic_device().capability_base(),
                has_triple_support: self
                    .has_capability(BreathEffect::INTERFACE, BreathEffect::METHOD_TRIPLE),
            })
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
    pub fn keyboard_layout(&self) -> Option<KeyboardLayout<'_>> {
        self.get_capability(
            KeyboardLayout,
            KeyboardLayout::INTERFACE,
            KeyboardLayout::METHOD,
        )
    }

    /// Controls the device's macro effect.
    pub fn macro_effect(&self) -> Option<MacroEffect<'_>> {
        self.get_capability(MacroEffect, MacroEffect::INTERFACE, MacroEffect::GET_METHOD)
    }

    /// Controls the device's Macro Mode.
    pub fn macro_mode(&self) -> Option<MacroMode<'_>> {
        self.get_capability(MacroMode, MacroMode::INTERFACE, MacroMode::GET_METHOD)
    }

    /// Disables any lighting effect.
    pub fn no_effect(&self) -> Option<NoEffect<'_>> {
        self.get_capability(NoEffect, NoEffect::INTERFACE, NoEffect::METHOD)
    }

    /// Controls the reactive effect.
    pub fn reactive_effect(&self) -> Option<ReactiveEffect<'_>> {
        self.get_capability(
            ReactiveEffect,
            ReactiveEffect::INTERFACE,
            ReactiveEffect::METHOD,
        )
    }

    /// Controls the ripple effect.
    pub fn ripple_effect(&self) -> Option<RippleEffect<'_>> {
        if self.has_capability(RippleEffect::INTERFACE, RippleEffect::METHOD) {
            Some(RippleEffect {
                base: self.get_generic_device().capability_base(),
                has_random_support: self
                    .has_capability(RippleEffect::INTERFACE, RippleEffect::METHOD_RANDOM),
            })
        } else {
            None
        }
    }

    /// Sets the spectrum effect.
    pub fn spectrum_effect(&self) -> Option<SpectrumEffect<'_>> {
        self.get_capability(
            SpectrumEffect,
            SpectrumEffect::INTERFACE,
            SpectrumEffect::METHOD,
        )
    }

    /// Sets the starlight effect.
    pub fn starlight_effect(&self) -> Option<StarlightEffect<'_>> {
        let has_single_support =
            self.has_capability(StarlightEffect::INTERFACE, StarlightEffect::METHOD_SINGLE);
        let has_random_support =
            self.has_capability(StarlightEffect::INTERFACE, StarlightEffect::METHOD_RANDOM);
        let has_dual_support =
            self.has_capability(StarlightEffect::INTERFACE, StarlightEffect::METHOD_DUAL);

        if has_single_support || has_random_support || has_dual_support {
            Some(StarlightEffect {
                base: self.get_generic_device().capability_base(),
                has_single_support,
                has_dual_support,
                has_random_support,
            })
        } else {
            None
        }
    }

    /// Controls the static effect.
    pub fn static_effect(&self) -> Option<StaticEffect<'_>> {
        self.get_capability(StaticEffect, StaticEffect::INTERFACE, StaticEffect::METHOD)
    }

    /// Controls the wave effect.
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
