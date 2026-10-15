//! # Lib.rs
//! 
//! Root mod file
//! 

// Folders
pub mod api;
pub mod metal;
pub(crate) mod modules;

// Re exports
pub use api::functions;
