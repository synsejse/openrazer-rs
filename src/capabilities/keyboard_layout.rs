use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

pub struct KeyboardLayout;

impl KeyboardLayout {
    pub(crate) const INTERFACE: &'static str = "razer.device.misc";
    pub(crate) const METHOD: &'static str = "getKeyboardLayout";

    pub(crate) async fn get(capability_base: CapabilityBase<'_>) -> Result<String, Error> {
        capability_base
            .call_get_method(Self::INTERFACE, Self::METHOD)
            .await
            .and_then(|response| match response {
                Value::String(layout) => Ok(layout),
                _ => Err(Error::InvalidResponse),
            })
    }
}
