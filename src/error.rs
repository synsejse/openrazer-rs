use dbus_async::DBusError;
use std::fmt::{self, Display, Formatter};

/// Errors that may occur when talking to the daemon.
#[derive(Debug)]
pub enum Error {
    /// A DBus error.
    DBusError(DBusError),
    /// The returned message failed to parse.
    InvalidResponse,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::DBusError(error) => Display::fmt(error, formatter),
            Error::InvalidResponse => formatter.write_str("Received invalid response"),
        }
    }
}

impl std::error::Error for Error {}

impl From<DBusError> for Error {
    fn from(error: DBusError) -> Self {
        Self::DBusError(error)
    }
}
