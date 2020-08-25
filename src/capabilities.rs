//! Various capabilities.

use crate::Error;
use dbus_async::DBus;
use dbus_message_parser::{Message, Value};

mod brightness;
mod game_mode;
mod keyboard_layout;
pub mod macro_effect;
mod macro_mode;

pub use brightness::Brightness;
pub use game_mode::GameMode;
pub(crate) use keyboard_layout::KeyboardLayout;
pub use macro_effect::MacroEffect;
pub use macro_mode::MacroMode;

#[derive(Debug, Clone)]
pub(crate) struct CapabilityBase<'a> {
    dbus: DBus,
    path: &'a str,
}

impl<'a> CapabilityBase<'a> {
    pub(crate) fn new(dbus: DBus, path: &'a str) -> Self {
        Self { dbus, path }
    }

    fn message(&self, interface: &'static str, method: &'static str) -> Message {
        Message::method_call(crate::DESTINATION, self.path, interface, method)
    }

    async fn call_get_method(
        &self,
        interface: &'static str,
        method: &'static str,
    ) -> Result<Value, Error> {
        let (_, body) = self
            .dbus
            .call(self.message(interface, method))
            .await?
            .split();

        if body.len() == 1 {
            Ok(body.into_iter().next().unwrap())
        } else {
            Err(Error::InvalidResponse)
        }
    }

    async fn call_set_method(
        &self,
        interface: &'static str,
        method: &'static str,
        value: Value,
    ) -> Result<(), Error> {
        let mut message = self.message(interface, method);
        message.add_value(value);
        let (_, body) = self.dbus.call(message).await?.split();

        if body.is_empty() {
            Ok(())
        } else {
            Err(Error::InvalidResponse)
        }
    }
}
