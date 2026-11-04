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
