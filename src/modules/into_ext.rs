//! # Into Extension
//! 
//! An extended version of `Into<T>`
//! 

/// `Into<T>` extended to accomodate
/// types that it normally doesn't
/// 
/// This is done with casting, so if the size
/// of `Self` doesn't fit inside `T` you
/// can lose information
pub trait IntoExtend<T: Sized> {
    /// Converts `Self` into `T`
    fn into_idx(&self) -> T;
}

impl IntoExtend<usize> for u8 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for u16 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for u32 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for u64 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for u128 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for i8 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for i16 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for i32 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for i64 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for i128 {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}

impl IntoExtend<usize> for isize {
    fn into_idx(&self) -> usize {
        *self as usize
    }
}
