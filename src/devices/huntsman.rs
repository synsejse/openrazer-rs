use super::{GenericDevice, GenericMethods};
use crate::capabilities::{
    BreathEffect, Brightness, GameMode, KeyboardLayout, MacroEffect, MacroMode, NoEffect,
    ReactiveEffect, RippleEffect, SpectrumEffect, StarlightEffect, StaticEffect, WaveEffect,
};

/// A Razer Huntsman keyboard.
#[derive(Debug, Clone)]
pub struct Huntsman(pub(crate) GenericDevice);

impl Huntsman {
    pub const NAME: &'static str = "Razer Huntsman";

    /// Controls the breath effect.
    pub fn breath_effect(&self) -> BreathEffect {
        BreathEffect {
            base: self.0.capability_base(),
            available: self.0.available.breath_effect,
        }
    }

    /// Controls brightness.
    pub fn brightness(&self) -> Brightness {
        Brightness(self.0.capability_base())
    }

    /// Controls Game Mode.
    pub fn game_mode(&self) -> GameMode {
        GameMode(self.0.capability_base())
    }

    /// Gets the keyboard's layout.
    pub fn keyboard_layout(&self) -> KeyboardLayout {
        KeyboardLayout(self.0.capability_base())
    }

    /// Controls the macro effect.
    pub fn macro_effect(&self) -> MacroEffect {
        MacroEffect(self.0.capability_base())
    }

    /// Controls Macro Mode.
    pub fn macro_mode(&self) -> MacroMode {
        MacroMode(self.0.capability_base())
    }

    /// Disables any lighting effect.
    pub fn no_effect(&self) -> NoEffect {
        NoEffect(self.0.capability_base())
    }

    /// Controls the reactive effect.
    pub fn reactive_effect(&self) -> ReactiveEffect {
        ReactiveEffect(self.0.capability_base())
    }

    /// Controls the ripple effect.
    pub fn ripple_effect(&self) -> RippleEffect {
        RippleEffect {
            base: self.0.capability_base(),
            available: self.0.available.ripple_effect,
        }
    }

    /// Sets the sprectrum effect.
    pub fn spectrum_effect(&self) -> SpectrumEffect {
        SpectrumEffect(self.0.capability_base())
    }

    /// Controls the starlight effect.
    pub fn starlight_effect(&self) -> StarlightEffect {
        StarlightEffect {
            base: self.0.capability_base(),
            available: self.0.available.starlight_effect,
        }
    }

    /// Controls the static effect.
    pub fn static_effect(&self) -> StaticEffect {
        StaticEffect(self.0.capability_base())
    }

    /// Controls the wave effect.
    pub fn wave_effect(&self) -> WaveEffect {
        WaveEffect(self.0.capability_base())
    }
}

impl GenericMethods for Huntsman {
    fn get_generic_device(&self) -> &GenericDevice {
        &self.0
    }
}
