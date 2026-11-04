#![doc = include_str!("../Readme.md")]
#![doc = include_str!("../docs/MIGRATION.md")]
//! Tokio-based, typed transport for the OpenRazer session-bus service.

mod client;
mod endpoint;
mod introspection;
mod method;
pub mod methods;

pub use client::Client;
pub use endpoint::Endpoint;
pub use introspection::Introspection;
pub use method::{Method, MethodInfo};

/// The daemon's well-known bus name.
pub const DESTINATION: &str = "org.razer";

/// The daemon's root object path; device objects live under `{ROOT}/device`.
pub const ROOT: &str = "/org/razer";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported method or signature: {0:?}")]
    Unsupported(MethodInfo),

    #[error("invalid device serial path component")]
    InvalidSerial,

    #[error("invalid introspection: {0}")]
    Introspection(#[from] zbus_xml::Error),

    #[error("D-Bus: {0}")]
    Bus(#[from] zbus::Error),

    #[error("D-Bus operation timed out")]
    Timeout,
}
