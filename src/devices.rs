mod generic;

pub use generic::{GenericDevice, GenericMethods};

#[derive(Debug, Clone)]
pub enum Device {
    // todo: Keyboard(Keyboard),
    // todo: Mouse(Mouse),
    // todo: Mousepad(Mousepad),
    Other(GenericDevice),
}

impl GenericMethods for Device {
    fn get_generic_device(&self) -> &GenericDevice {
        match self {
            Self::Other(other) => &other,
        }
    }
}
