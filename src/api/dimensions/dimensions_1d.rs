//! # Dimensions
//! 
//! Describes a set of dimensions
//! 

use std::ops::Mul;

use crate::modules::{dimensional::Dimensional, primitive::UPrimitive};

/// The size of a single dimension
pub struct Dimensions1D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    length: T,
}

impl<T> Default for Dimensions1D<T>
where
    T: UPrimitive + Copy + Mul<Output = T> + Default,
{
    fn default() -> Self {
        Self {
            length: T::default(),
        }
    }
}

impl<T> Dimensional for Dimensions1D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    type Output = T;

    fn get_size(&self) -> Self::Output {
        self.length
    }
}

impl<T> Dimensions1D<T>
where
    T: UPrimitive + Copy + Mul<Output = T>,
{
    /// Creates a new `Self` from dimensions
    pub fn from_dimensions(
        length: T,
    ) -> Self {
        Self {
            length,
        }
    }

    /// Sets the length of the dimension
    /// 
    /// Because there is only one dimension
    /// this is the same as setting the size
    pub fn set_length(&mut self, length: T) {
        self.length = length;
    }
}
