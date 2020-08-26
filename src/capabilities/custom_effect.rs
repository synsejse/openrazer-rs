use super::{available::CustomEffectAvailability, matrix_dimensions::Dimensions, CapabilityBase};
use crate::{Color, Error};
use dbus_message_parser::Value;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct CustomEffect {
    pub(crate) base: CapabilityBase,
    pub(crate) availability: CustomEffectAvailability,
}

#[derive(Debug, Clone)]
pub struct Frame {
    effect: CustomEffect,
    dimensions: Dimensions,
    buffer: Vec<Color>,
}

impl CustomEffect {
    pub(crate) const INTERFACE: &'static str = "razer.device.lighting.chroma";
    pub(crate) const SET_ROW_METHOD: &'static str = "setKeyRow";
    pub(crate) const SET_CUSTOM_METHOD: &'static str = "setCustom";

    pub(crate) async fn set_row(&self, bytes: Vec<Value>) -> Result<(), Error> {
        if !self.availability.set_key_row {
            return Err(Error::UnsupportedCapability);
        }

        self.base
            .call_set_method(
                Self::INTERFACE,
                Self::SET_ROW_METHOD,
                vec![Value::Array(bytes, "y".to_string())],
            )
            .await
    }

    pub async fn set(&self) -> Result<(), Error> {
        if !self.availability.set_effect {
            return Err(Error::UnsupportedCapability);
        }

        self.base
            .call_set_method(Self::INTERFACE, Self::SET_CUSTOM_METHOD, vec![])
            .await
    }

    pub fn frame(self, dimensions: Dimensions) -> Frame {
        Frame {
            effect: self,
            dimensions,
            buffer: vec![(0, 0, 0); dimensions.rows * dimensions.columns],
        }
    }
}

impl Frame {
    pub async fn draw(&self) -> Result<(), Error> {
        let mut bytes = Vec::with_capacity(self.dimensions.rows * (self.dimensions.columns + 3));

        for row_index in 0..self.dimensions.rows {
            bytes.push(Value::Byte(row_index as u8));
            bytes.push(Value::Byte(0));
            bytes.push(Value::Byte((self.dimensions.columns - 1) as u8));
            let row = &self.buffer
                [row_index * self.dimensions.columns..(row_index + 1) * self.dimensions.columns];
            bytes.extend(row.iter().flat_map(|(red, green, blue)| {
                vec![Value::Byte(*red), Value::Byte(*green), Value::Byte(*blue)]
            }));
        }

        self.effect.set_row(bytes).await
    }
}

impl Index<(usize, usize)> for Frame {
    type Output = Color;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(
            row < self.dimensions.rows,
            "row is {} but dimensions are {:?}",
            row,
            self.dimensions
        );
        assert!(
            column < self.dimensions.columns,
            "column is {} but dimensions are {:?}",
            column,
            self.dimensions
        );

        &self.buffer[(row * self.dimensions.columns + column)]
    }
}

impl IndexMut<(usize, usize)> for Frame {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(
            row < self.dimensions.rows,
            "row is {} but dimensions are {:?}",
            row,
            self.dimensions
        );
        assert!(
            column < self.dimensions.columns,
            "column is {} but dimensions are {:?}",
            column,
            self.dimensions
        );

        &mut self.buffer[(row * self.dimensions.columns + column)]
    }
}
