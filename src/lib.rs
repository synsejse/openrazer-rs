#![doc = include_str!("../Readme.md")]
#![doc = include_str!("../docs/MIGRATION.md")]
//! Tokio-based, typed transport for the OpenRazer session-bus service.
use serde::Serialize;
use std::time::Duration;
use zbus::{
    Connection,
    wire::{DynamicDeserialize, DynamicType},
};

pub const DESTINATION: &str = "org.razer";
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

/// A cloneable connection; never starts the daemon or retries setters.
#[derive(Debug, Clone)]
pub struct Client {
    connection: Connection,
    timeout: Duration,
}
impl Client {
    pub async fn connect() -> Result<Self, Error> {
        let connection = tokio::time::timeout(Duration::from_secs(10), Connection::session())
            .await
            .map_err(|_| Error::Timeout)??;
        Ok(Self::from_connection(connection))
    }
    /// Inject an existing connection (including a private test bus).
    pub fn from_connection(connection: Connection) -> Self {
        Self {
            connection,
            timeout: Duration::from_secs(10),
        }
    }
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn connection(&self) -> &Connection {
        &self.connection
    }
    /// Low-level escape hatch: does not check advertised capabilities.
    /// Prefer [`Endpoint::call`] for audited methods.
    pub async fn call<B: Serialize + DynamicType, R: for<'de> DynamicDeserialize<'de>>(
        &self,
        path: &str,
        interface: &str,
        method: &str,
        body: &B,
    ) -> Result<R, Error> {
        tokio::time::timeout(self.timeout, async {
            let proxy = zbus::proxy::Builder::<zbus::Proxy<'_>>::new(&self.connection)
                .destination(DESTINATION)
                .path(path)
                .interface(interface)
                .cache_properties(zbus::proxy::CacheProperties::No)
                .build()
                .await?;
            proxy
                .call_with_flags(method, zbus::proxy::MethodFlags::NO_AUTO_START, body)
                .await?
                .ok_or(zbus::Error::Failure("missing reply".into()))
        })
        .await
        .map_err(|_| Error::Timeout)?
        .map_err(Error::Bus)
    }
    pub async fn serials(&self) -> Result<Vec<String>, Error> {
        self.call(ROOT, "razer.devices", "getDevices", &()).await
    }
    pub async fn daemon_version(&self) -> Result<String, Error> {
        self.call(ROOT, "razer.daemon", "version", &()).await
    }
}

mod introspection;
pub mod methods;
pub use introspection::Introspection;

/// Exact D-Bus argument-list signatures (outer tuples are not wire structs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodInfo {
    pub interface: &'static str,
    pub name: &'static str,
    pub input: &'static str,
    pub output: &'static str,
}
#[derive(Debug)]
pub struct Method<I, O> {
    info: MethodInfo,
    marker: std::marker::PhantomData<fn(I) -> O>,
}
impl<I, O> Method<I, O> {
    const fn new(
        interface: &'static str,
        name: &'static str,
        input: &'static str,
        output: &'static str,
    ) -> Self {
        Self {
            info: MethodInfo {
                interface,
                name,
                input,
                output,
            },
            marker: std::marker::PhantomData,
        }
    }
    pub const fn info(&self) -> MethodInfo {
        self.info
    }
}

/// An introspected root or device object. Recreate after hotplug/owner changes.
#[derive(Debug)]
pub struct Endpoint {
    client: Client,
    path: String,
    capabilities: Introspection,
}
impl Endpoint {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn capabilities(&self) -> &Introspection {
        &self.capabilities
    }
    /// Checks both signatures before dispatch. Setters are never retried;
    /// successful acknowledgement does not establish hardware readback.
    pub async fn call<I: Serialize + DynamicType, O: for<'de> DynamicDeserialize<'de>>(
        &self,
        method: &Method<I, O>,
        body: &I,
    ) -> Result<O, Error> {
        if !self.capabilities.supports(method) {
            return Err(Error::Unsupported(method.info()));
        }
        let m = method.info();
        self.client
            .call(&self.path, m.interface, m.name, body)
            .await
    }
}
impl Client {
    pub async fn endpoint(&self, path: &str) -> Result<Endpoint, Error> {
        let xml: String = self
            .call(
                path,
                "org.freedesktop.DBus.Introspectable",
                "Introspect",
                &(),
            )
            .await?;
        Ok(Endpoint {
            client: self.clone(),
            path: path.to_owned(),
            capabilities: Introspection::parse(&xml)?,
        })
    }
    pub async fn root(&self) -> Result<Endpoint, Error> {
        self.endpoint(ROOT).await
    }
    pub async fn device(&self, serial: &str) -> Result<Endpoint, Error> {
        if serial.is_empty()
            || !serial
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_')
        {
            return Err(Error::InvalidSerial);
        }
        self.endpoint(&format!("{ROOT}/device/{serial}")).await
    }
    /// Hotplug notifications have no serial argument. Re-enumerate on either signal.
    pub async fn device_events(&self) -> Result<zbus::MessageStream, Error> {
        let rule = zbus::MatchRule::builder()
            .msg_type(zbus::message::Type::Signal)
            .sender(DESTINATION)
            .interface("razer.devices")
            .path(ROOT)
            .build()?;
        Ok(zbus::MessageStream::for_match_rule(
            rule,
            &self.connection,
            std::num::NonZeroUsize::new(16),
        )
        .await?)
    }
    /// Observe daemon replacement/disappearance, then discard capability snapshots.
    pub async fn owner_events(&self) -> Result<zbus::MessageStream, Error> {
        let rule = zbus::MatchRule::builder()
            .msg_type(zbus::message::Type::Signal)
            .sender("org.freedesktop.DBus")
            .interface("org.freedesktop.DBus")
            .path("/org/freedesktop/DBus")
            .member("NameOwnerChanged")
            .arg(0, DESTINATION)
            .build()?;
        Ok(zbus::MessageStream::for_match_rule(
            rule,
            &self.connection,
            std::num::NonZeroUsize::new(16),
        )
        .await?)
    }
}
