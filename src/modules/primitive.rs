//! # Primitives
//! 
//! Traits describing what counts as a primitive
//! 

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// All primitives
pub trait Primitive
where
    Self: Sized
        + Add + AddAssign
        + Sub + SubAssign
        + Mul + MulAssign
        + Div + DivAssign,
{}

/// Unsigned primitives
pub trait UPrimitive
where
    Self: Sized
        + Add + AddAssign
        + Sub + SubAssign
        + Mul + MulAssign
        + Div + DivAssign,
{}

/// Signed primitives
pub trait IPrimitive
where
    Self: Sized
        + Add + AddAssign
        + Sub + SubAssign
        + Mul + MulAssign
        + Div + DivAssign,
{}

macro_rules! impl_primitives {
    ($trait_name:ident: $($type:ty),+ $(,)?) => {
        $(impl $trait_name for $type {})+
    };
}

impl_primitives!(Primitive: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_primitives!(UPrimitive: u8, u16, u32, u64, u128, usize);
impl_primitives!(IPrimitive: i8, i16, i32, i64, i128, isize, f32, f64);
