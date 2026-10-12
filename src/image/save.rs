//! # Save
//! 
//! Save a buffer as an image
//! 

pub trait SaveImage {
    /// Save a buffer as a png
    fn save_png(&self);
}
