//! # Dimensional
//! 
//! Trait describing the methods a dimension should have
//! 

/// Add docs later
pub trait Dimensional {
    type Output;

    /// Gets the total size of
    /// all the dimensions together
    fn get_size(&self) -> Self::Output;
}
