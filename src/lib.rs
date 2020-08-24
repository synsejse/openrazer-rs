pub mod devices;
mod device_manager;
mod error;

pub use devices::Device;
pub use device_manager::DeviceManager;
pub use error::Error;

pub(crate) const DESTINATION: &str = "org.razer";
