use super::CapabilityBase;
use crate::Error;

/// Controls the spectrum effect.
#[derive(Debug, Clone)]
pub struct SpectrumEffect<'a>(pub(crate) CapabilityBase<'a>);

impl SpectrumEffect<'_> {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD: &'static str = "setSpectrum";

    /// Sets the spectrum effect.
    pub async fn set(&self) -> Result<(), Error> {
        self.0
            .call_set_method(Self::INTERFACE, Self::METHOD, vec![])
            .await
    }
}
