use super::CapabilityBase;
use crate::Error;

/// Disables any lighting effect.
#[derive(Debug, Clone)]
pub struct NoEffect<'a>(pub(crate) CapabilityBase<'a>);

impl NoEffect<'_> {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD: &'static str = "setNone";

    /// Disables any lighting effect
    pub async fn set(&self) -> Result<(), Error> {
        self.0
            .call_set_method(Self::INTERFACE, Self::METHOD, vec![])
            .await
    }
}
