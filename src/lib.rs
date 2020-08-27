pub mod capabilities;
pub mod device_manager;
pub mod devices;
mod error;

pub use device_manager::DeviceManager;
pub use devices::Device;
pub use error::Error;

pub(crate) const DESTINATION: &str = "org.razer";

pub type Color = (u8, u8, u8);
