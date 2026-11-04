use serde::Serialize;
use zbus::wire::{DynamicDeserialize, DynamicType};

use crate::{Client, Error, Introspection, Method};

/// An introspected root or device object. Recreate after hotplug/owner changes.
#[derive(Debug)]
pub struct Endpoint {
    client: Client,
    path: String,
    capabilities: Introspection,
}

impl Endpoint {
    pub(crate) fn new(client: Client, path: String, capabilities: Introspection) -> Self {
        Self {
            client,
            path,
            capabilities,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn capabilities(&self) -> &Introspection {
        &self.capabilities
    }

    /// Checks both signatures before dispatch. Setters are never retried;
    /// successful acknowledgement does not establish hardware readback.
    pub async fn call<I, O>(&self, method: &Method<I, O>, body: &I) -> Result<O, Error>
    where
        I: Serialize + DynamicType,
        O: for<'de> DynamicDeserialize<'de>,
    {
        if !self.capabilities.supports(method) {
            return Err(Error::Unsupported(method.info()));
        }
        let info = method.info();
        self.client
            .call(&self.path, info.interface, info.name, body)
            .await
    }
}
