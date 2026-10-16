//! # Pixel
//! 
//! A pixel on the screen
//! 

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::metal::metal_format::MetalFormat;

/// A representation
/// the bgra values for
/// a pixel
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pixel<B>
where
    B: Copy + MetalFormat,
{
    /// Blue
    b: B,
    /// Green
    g: B,
    /// Red
    r: B,
    /// Alpha
    a: B,
}

impl<B> Add for Pixel<B>
where
    B: Copy + MetalFormat + Add<Output = B>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_alpha(
            self.get_red() + rhs.get_red(),
            self.get_green() + rhs.get_green(),
            self.get_blue() + rhs.get_blue(),
            self.get_alpha() + rhs.get_alpha(),
        )
    }
}

impl<B> Sub for Pixel<B>
where
    B: Copy + MetalFormat + Sub<Output = B>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_alpha(
            self.get_red() - rhs.get_red(),
            self.get_green() - rhs.get_green(),
            self.get_blue() - rhs.get_blue(),
            self.get_alpha() - rhs.get_alpha(),
        )
    }
}

impl<B> Mul for Pixel<B>
where
    B: Copy + MetalFormat + Mul<Output = B>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_alpha(
            self.get_red() * rhs.get_red(),
            self.get_green() * rhs.get_green(),
            self.get_blue() * rhs.get_blue(),
            self.get_alpha() * rhs.get_alpha(),
        )
    }
}

impl<B> Div for Pixel<B>
where
    B: Copy + MetalFormat + Div<Output = B>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new_alpha(
            self.get_red() / rhs.get_red(),
            self.get_green() / rhs.get_green(),
            self.get_blue() / rhs.get_blue(),
            self.get_alpha() / rhs.get_alpha(),
        )
    }
}

impl<B> AddAssign for Pixel<B>
where
    B: Copy + MetalFormat + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl<B> SubAssign for Pixel<B>
where
    B: Copy + MetalFormat + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl<B> MulAssign for Pixel<B>
where
    B: Copy + MetalFormat + MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl<B> DivAssign for Pixel<B>
where
    B: Copy + MetalFormat + DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}



impl<B> Default for Pixel<B>
where
    B: Copy + MetalFormat + Default,
{
    fn default() -> Self {
        Self {
            r: B::default(),
            g: B::default(),
            b: B::default(),
            a: B::MAX,
        }
    }
}

impl<B> From<B> for Pixel<B>
where
    B: Copy + MetalFormat,
{
    fn from(value: B) -> Self {
        Self {
            r: value,
            g: value,
            b: value,
            a: value,
        }
    }
}

impl<B> Pixel<B>
where
    B: Copy + MetalFormat,
{
    /// Creates a new `Pixel`
    /// with the `MAX` value of `B`
    /// for the alpha channel, or 100% opacity
    pub fn new(
        red: B,
        green: B,
        blue: B,
    ) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: B::MAX,
        }
    }

    /// Creates a new pixel
    /// with an opacity
    pub fn new_alpha(
        red: B,
        green: B,
        blue: B,
        alpha: B,
    ) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    /// Gets the pixels `RED` value
    pub fn get_red(&self) -> B {
        self.r
    }

    /// Gets the pixels `BLUE` value
    pub fn get_blue(&self) -> B {
        self.b
    }

    /// Gets the pixels `GREEN` value
    pub fn get_green(&self) -> B {
        self.g
    }

    /// Gets the pixels `ALPHA` value
    pub fn get_alpha(&self) -> B {
        self.a
    }

    /// Sets the pixels `RED` value
    pub fn set_red(&mut self, red: B) {
        self.r = red;
    }

    /// Sets the pixels `BLUE` value
    pub fn set_blue(&mut self, blue: B) {
        self.b = blue;
    }

    /// Sets the pixels `GREEN` value
    pub fn set_green(&mut self, green: B) {
        self.g = green;
    }

    /// Sets the pixels `ALPHA` value
    pub fn set_alpha(&mut self, alpha: B) {
        self.a = alpha;
    }
}
