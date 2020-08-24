mod generic;
pub mod keyboard;

pub use generic::{GenericDevice, GenericMethods};
pub use keyboard::Keyboard;

#[derive(Debug, Clone)]
pub enum Device {
    Keyboard(Keyboard),
    // todo: Mouse(Mouse),
    // todo: Mousepad(Mousepad),
    Other(GenericDevice),
}

impl GenericMethods for Device {
    fn get_generic_device(&self) -> &GenericDevice {
        match self {
            Self::Keyboard(keyboard) => &keyboard.get_generic_device(),
            Self::Other(other) => &other,
        }
    }
}
