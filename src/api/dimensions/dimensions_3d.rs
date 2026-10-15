//! # Dimensions
//! 
//! Describes a set of dimensions
//! 

use std::ops::Mul;

use crate::modules::{dimensional::Dimensional, primitive::UPrimitive};

/// The size of 3 dimension
pub struct Dimensions3D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    x: T,
    y: T,
    z: T
}

impl<T> Default for Dimensions3D<T>
where
    T: UPrimitive + Copy + Mul<Output = T> + Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> Dimensional for Dimensions3D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    type Output = T;

    fn get_size(&self) -> Self::Output {
        self.x * self.y * self.z
    }
}

impl<T> From<(T, T, T)> for Dimensions3D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T> Dimensions3D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    /// Creates a new `Self` from dimensions
    pub fn from_dimensions(
        x: T,
        y: T,
        z: T,
    ) -> Self {
        Self {
            x,
            y,
            z,
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

    /// Sets the length of the z dimension
    pub fn set_z_length(&mut self, z: T) {
        self.z = z;
    }

    /// Gets the length of the z dimension
    pub fn get_z_length(& self) -> T {
        self.z
    }
}
