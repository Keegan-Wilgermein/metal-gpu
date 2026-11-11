//! Simple debugging
//! 
//! Appends a debug line to a variable, then returns it
//! 

use std::{fmt::Debug, path::PathBuf};

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

impl Debugger for bool {}
impl Debugger for PathBuf {}
impl<T> Debugger for Option<T> where T: Debug, {}
impl<T, E> Debugger for Result<T, E> where T: Debug, E: Debug {}
