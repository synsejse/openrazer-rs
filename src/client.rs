use std::{num::NonZeroUsize, time::Duration};

use serde::Serialize;
use zbus::{
    Connection, MatchRule, MessageStream,
    message::Type,
    proxy::{Builder, CacheProperties, MethodFlags},
    wire::{DynamicDeserialize, DynamicType},
};

use crate::{DESTINATION, Endpoint, Error, Introspection, ROOT};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
const SIGNAL_QUEUE_CAPACITY: Option<NonZeroUsize> = NonZeroUsize::new(16);

/// A cloneable connection; never starts the daemon or retries setters.
#[derive(Debug, Clone)]
pub struct Client {
    connection: Connection,
    timeout: Duration,
}

impl Client {
    pub async fn connect() -> Result<Self, Error> {
        let connection = tokio::time::timeout(DEFAULT_TIMEOUT, Connection::session())
            .await
            .map_err(|_| Error::Timeout)??;
        Ok(Self::from_connection(connection))
    }

    /// Inject an existing connection (including a private test bus).
    pub fn from_connection(connection: Connection) -> Self {
        Self {
            connection,
            timeout: DEFAULT_TIMEOUT,
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
    pub async fn call<B, R>(
        &self,
        path: &str,
        interface: &str,
        method: &str,
        body: &B,
    ) -> Result<R, Error>
    where
        B: Serialize + DynamicType,
        R: for<'de> DynamicDeserialize<'de>,
    {
        tokio::time::timeout(self.timeout, async {
            let proxy = Builder::<zbus::Proxy<'_>>::new(&self.connection)
                .destination(DESTINATION)
                .path(path)
                .interface(interface)
                .cache_properties(CacheProperties::No)
                .build()
                .await?;
            proxy
                .call_with_flags(method, MethodFlags::NO_AUTO_START, body)
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

    /// Introspect an object once; see [`Endpoint`] for the snapshot's lifetime.
    pub async fn endpoint(&self, path: &str) -> Result<Endpoint, Error> {
        let xml: String = self
            .call(
                path,
                "org.freedesktop.DBus.Introspectable",
                "Introspect",
                &(),
            )
            .await?;
        let capabilities = Introspection::parse(&xml)?;
        Ok(Endpoint::new(self.clone(), path.to_owned(), capabilities))
    }

    pub async fn root(&self) -> Result<Endpoint, Error> {
        self.endpoint(ROOT).await
    }

    pub async fn device(&self, serial: &str) -> Result<Endpoint, Error> {
        if !is_path_component(serial) {
            return Err(Error::InvalidSerial);
        }
        self.endpoint(&format!("{ROOT}/device/{serial}")).await
    }

    /// Hotplug notifications have no serial argument. Re-enumerate on either signal.
    pub async fn device_events(&self) -> Result<MessageStream, Error> {
        let rule = MatchRule::builder()
            .msg_type(Type::Signal)
            .sender(DESTINATION)
            .interface("razer.devices")
            .path(ROOT)
            .build()?;
        self.signals(rule).await
    }

    /// Observe daemon replacement/disappearance, then discard capability snapshots.
    pub async fn owner_events(&self) -> Result<MessageStream, Error> {
        let rule = MatchRule::builder()
            .msg_type(Type::Signal)
            .sender("org.freedesktop.DBus")
            .interface("org.freedesktop.DBus")
            .path("/org/freedesktop/DBus")
            .member("NameOwnerChanged")
            .arg(0, DESTINATION)
            .build()?;
        self.signals(rule).await
    }

    async fn signals(&self, rule: MatchRule<'_>) -> Result<MessageStream, Error> {
        let stream =
            MessageStream::for_match_rule(rule, &self.connection, SIGNAL_QUEUE_CAPACITY).await?;
        Ok(stream)
    }
}

/// Whether `serial` can be one D-Bus object-path element.
fn is_path_component(serial: &str) -> bool {
    !serial.is_empty()
        && serial
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}
