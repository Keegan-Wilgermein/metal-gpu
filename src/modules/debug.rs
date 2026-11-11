//! Simple debugging
//! 
//! Appends a debug line to a variable, then returns it
//! 

use std::fmt::Debug;

/// Checks for validity or prints the value
#[allow(unused)]
pub(crate) trait Debugger
where
    Self: Debug + Sized,
{
    /// Prints the value of a variable
    fn debug(self) -> Self {
        println!("{:#?}", self);
        self
    }
}

impl<T> Debugger for T where T: Debug {}
