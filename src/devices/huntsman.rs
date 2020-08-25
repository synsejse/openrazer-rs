use super::{GenericDevice, GenericMethods};
use crate::{
    capabilities::{
        BreathEffect, Brightness, GameMode, KeyboardLayout, MacroEffect, MacroMode, NoEffect,
        ReactiveEffect, SpectrumEffect, StaticEffect, WaveEffect,
    },
    Error,
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
    pub async fn keyboard_layout(&self) -> Result<String, Error> {
        KeyboardLayout::get(self.0.capability_base()).await
    }

    /// Controls the macro effect.
    pub fn macro_effect(&self) -> MacroEffect<'_> {
        MacroEffect(self.0.capability_base())
    }

    /// Controls Macro Mode.
    pub fn macro_mode(&self) -> MacroMode<'_> {
        MacroMode(self.0.capability_base())
    }

    /// Removes any lighting effect.
    pub async fn disable_lighting(&self) -> Result<(), Error> {
        NoEffect::set(self.0.capability_base()).await
    }

    /// Controls the reactive effect.
    pub async fn reactive_effect(&self) -> ReactiveEffect<'_> {
        ReactiveEffect(self.0.capability_base())
    }

    /// Sets the sprectrum effect.
    pub async fn spectrum_effect(&self) -> Result<(), Error> {
        SpectrumEffect::set(self.0.capability_base()).await
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
