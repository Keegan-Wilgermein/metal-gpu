//! # Pixel
//! 
//! A pixel on the screen
//! 

/// A black pixel
pub const BLACK: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 1.0,
};

/// A white pixel
/// 
/// Same as default
pub const WHITE: Pixel = Pixel {
    r: u16::MAX,
    g: u16::MAX,
    b: u16::MAX,
    a: 1.0,
};

/// A red pixel
pub const RED: Pixel = Pixel {
    r: u16::MAX,
    g: 0,
    b: 0,
    a: 1.0,
};

/// A green pixel
pub const GREEN: Pixel = Pixel {
    r: 0,
    g: u16::MAX,
    b: 0,
    a: 1.0,
};

/// A blue pixel
pub const BLUE: Pixel = Pixel {
    r: 0,
    g: 0,
    b: u16::MAX,
    a: 1.0,
};

/// A representation
/// the rgba values for
/// a single pixel
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pixel {
    /// Red
    r: u16,
    /// Green
    g: u16,
    /// Blue
    b: u16,

    /// Alpha between 0 and 1
    a: f32,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: u16::MAX,
            g: u16::MAX,
            b: u16::MAX,
            a: 1.0,
        }
    }
}

impl From<u16> for Pixel {
    fn from(value: u16) -> Self {
        Self {
            r: value,
            g: value,
            b: value,
            ..Default::default()
        }
    }
}

impl Pixel {
    /// Creates a new `Pixel`
    /// with `1.0` opacity
    pub fn new(
        red: u16,
        green: u16,
        blue: u16,
    ) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            ..Default::default()
        }
    }

    /// Creates a new pixel
    /// with an opacity
    pub fn new_alpha(
        red: u16,
        green: u16,
        blue: u16,
        alpha: f32,
    ) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    /// Gets the pixels `RED` value
    pub fn get_red(&self) -> u16 {
        self.r
    }

    /// Gets the pixels `BLUE` value
    pub fn get_blue(&self) -> u16 {
        self.b
    }

    /// Gets the pixels `GREEN` value
    pub fn get_green(&self) -> u16 {
        self.g
    }

    /// Gets the pixels `ALPHA` value
    pub fn get_alpha(&self) -> f32 {
        self.a
    }

    /// Gets the pixels `RED` value
    pub fn set_red(&mut self, red: u16) {
        self.r = red;
    }

    /// Gets the pixels `BLUE` value
    pub fn set_blue(&mut self, blue: u16) {
        self.b = blue;
    }

    /// Gets the pixels `GREEN` value
    pub fn set_green(&mut self, green: u16) {
        self.g = green;
    }

    /// Gets the pixels `ALPHA` value,
    /// clamping between `0.0` and `1.0`
    pub fn set_alpha(&mut self, alpha: f32) {
        self.a = alpha.clamp(0.0, 1.0);
    }
}
