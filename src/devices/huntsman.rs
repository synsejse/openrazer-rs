use super::{GenericDevice, GenericMethods};
use crate::{
    capabilities::{
        BreathEffect, Brightness, GameMode, KeyboardLayout, MacroEffect, MacroMode, NoEffect,
        ReactiveEffect, SpectrumEffect, StaticEffect, WaveEffect,
    },
};

/// A Razer Huntsman keyboard.
#[derive(Debug, Clone)]
pub struct Huntsman(pub(crate) GenericDevice);

impl Huntsman {
    pub const NAME: &'static str = "Razer Huntsman";

    /// Controls the breath effect.
    pub fn breath_effect(&self) -> BreathEffect<'_> {
        BreathEffect {
            base: self.0.capability_base(),
            has_triple_support: false,
        }
    }

    /// Controls brightness.
    pub fn brightness(&self) -> Brightness<'_> {
        Brightness(self.0.capability_base())
    }

    /// Controls Game Mode.
    pub fn game_mode(&self) -> GameMode<'_> {
        GameMode(self.0.capability_base())
    }

    /// Gets the keyboard's layout.
    pub fn keyboard_layout(&self) -> KeyboardLayout<'_> {
        KeyboardLayout(self.0.capability_base())
    }

    /// Controls the macro effect.
    pub fn macro_effect(&self) -> MacroEffect<'_> {
        MacroEffect(self.0.capability_base())
    }

    /// Controls Macro Mode.
    pub fn macro_mode(&self) -> MacroMode<'_> {
        MacroMode(self.0.capability_base())
    }

    /// Disables any lighting effect.
    pub fn no_effect(&self) -> NoEffect<'_> {
        NoEffect(self.0.capability_base())
    }

    /// Controls the reactive effect.
    pub fn reactive_effect(&self) -> ReactiveEffect<'_> {
        ReactiveEffect(self.0.capability_base())
    }

    /// Sets the sprectrum effect.
    pub fn spectrum_effect(&self) -> SpectrumEffect<'_> {
        SpectrumEffect(self.0.capability_base())
    }

    /// Controls the static effect.
    pub fn static_effect(&self) -> StaticEffect<'_> {
        StaticEffect(self.0.capability_base())
    }

    /// Controls the wave effect.
    pub fn wave_effect(&self) -> WaveEffect<'_> {
        WaveEffect(self.0.capability_base())
    }
}

impl GenericMethods for Huntsman {
    fn get_generic_device(&self) -> &GenericDevice {
        &self.0
    }
}
