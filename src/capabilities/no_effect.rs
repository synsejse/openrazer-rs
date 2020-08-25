use super::CapabilityBase;
use crate::Error;

pub struct NoEffect;

impl NoEffect {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const METHOD: &'static str = "setNone";

    pub(crate) async fn set(capability_base: CapabilityBase<'_>) -> Result<(), Error> {
        capability_base
            .call_set_method(Self::INTERFACE, Self::METHOD, vec![])
            .await
    }
}
