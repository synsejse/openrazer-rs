use super::CapabilityBase;
use crate::Error;
use dbus_message_parser::Value;
use std::convert::TryInto;

pub struct MatrixDimensions(pub(crate) CapabilityBase);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Dimensions {
    pub rows: usize,
    pub columns: usize,
}

impl MatrixDimensions {
    pub(crate) const INTERFACE: &'static str = "razer.device.misc";
    pub(crate) const GET_METHOD: &'static str = "getMatrixDimensions";

    pub async fn get(&self) -> Result<Dimensions, Error> {
        self.0
            .call_get_method(Self::INTERFACE, Self::GET_METHOD)
            .await
            .and_then(|response| match response {
                Value::Array(dimensions, _) => match dimensions.as_slice() {
                    [Value::Int32(rows), Value::Int32(columns)] => {
                        let rows = (*rows).try_into().or(Err(Error::InvalidResponse))?;
                        let columns = (*columns).try_into().or(Err(Error::InvalidResponse))?;
                        Ok(Dimensions { rows, columns })
                    }
                    _ => Err(Error::InvalidResponse),
                },
                _ => Err(Error::InvalidResponse),
            })
    }
}
