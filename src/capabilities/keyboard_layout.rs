use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;

/// Gets a device's keyboard layout.
#[derive(Debug, Clone)]
pub struct KeyboardLayout(pub(crate) CapabilityBase);

impl KeyboardLayout {
    pub(crate) const INTERFACE: &'static str = "razer.device.misc";
    pub(crate) const METHOD: &'static str = "getKeyboardLayout";

    pub async fn get(&self) -> Result<String, Error> {
        self.0
            .call_get_method(Self::INTERFACE, Self::METHOD)
            .await
            .and_then(|response| match response {
                Value::String(layout) => Ok(layout),
                _ => Err(Error::InvalidResponse),
            })
    }
}
