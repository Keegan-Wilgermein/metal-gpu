//! # Byte Alignment
//! 
//! Trait for checking byte alignment
//! for different types and conditions
//! 

/// Checks for correct byte alignment
pub trait ByteAlignment
where
    Self: Sized,
{
    /// Checks that `total` is a multiple
    /// of the alignment in `self`
    ///
    /// Returns `total` if it's aligned,
    /// otherwise `None`
    fn check_alignment(
        &self,
        total: usize,
    ) -> Result<Self, ()>;
}

impl ByteAlignment for usize {
    fn check_alignment(
        &self,
        total: usize,
    ) -> Result<Self, ()> {
        match total.checked_rem(*self) {
            Some(0) => Ok(total),
            _ => Err(()),
        }
    }
}
