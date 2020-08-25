use super::{GenericDevice, GenericMethods};
use crate::{
    capabilities::{
        Brightness, GameMode, KeyboardLayout, MacroEffect, MacroMode, NoEffect, WaveEffect,
    },
    Error,
};

/// A Razer Huntsman keyboard.
#[derive(Debug, Clone)]
pub struct Huntsman(pub(crate) GenericDevice);

impl Huntsman {
    pub const NAME: &'static str = "Razer Huntsman";

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
