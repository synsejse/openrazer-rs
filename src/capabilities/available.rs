/// A list of available capabilities.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct AvailableCapabilities {
    pub breath_effect: BreathEffectAvailability,
    pub brightness: bool,
    pub game_mode: bool,
    pub keyboard_layout: bool,
    pub macro_effect: bool,
    pub macro_mode: bool,
    pub no_effect: bool,
    pub reactive_effect: bool,
    pub ripple_effect: RippleEffectAvailability,
    pub spectrum_effect: bool,
    pub starlight_effect: StarlightEffectAvailability,
    pub static_effect: bool,
    pub wave_effect: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct BreathEffectAvailability {
    pub single: bool,
    pub dual: bool,
    pub triple: bool,
    pub random: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct RippleEffectAvailability {
    pub single: bool,
    pub random: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct StarlightEffectAvailability {
    pub single: bool,
    pub dual: bool,
    pub random: bool,
}
