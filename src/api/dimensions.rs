//! # Dimensions
//! 
//! Describes a set of dimensions
//! 

use std::{ops::Mul};

use crate::modules::{primitive::UPrimitive};

/// The size of 2 dimension
pub struct Dimensions<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    x: T,
    y: T,
}

impl<T> Default for Dimensions<T>
where
    T: UPrimitive + Copy + Mul<Output = T> + Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> From<T> for Dimensions<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    fn from(value: T) -> Self {
        Self {
            x: value,
            y: value,
        }
    }
}

impl<T> From<(T, T)> for Dimensions<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> Dimensions<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    /// Creates a new `Self` from dimensions
    pub fn from_dimensions(
        x: T,
        y: T,
    ) -> Self {
        Self {
            x,
            y,
        }
    }

    /// Sets the length of the x dimension
    pub fn set_x_length(&mut self, x: T) {
        self.x = x;
    }

    /// Gets the length of the x dimension
    pub fn get_x_length(& self) -> T {
        self.x
    }

    /// Sets the length of the y dimension
    pub fn set_y_length(&mut self, y: T) {
        self.y = y;
    }

    /// Gets the length of the y dimension
    pub fn get_y_length(& self) -> T {
        self.y
    }

    /// Gets the combined size of
    /// the dimensions
    pub fn get_size(&self) -> T {
        self.x * self.y
    }
}
