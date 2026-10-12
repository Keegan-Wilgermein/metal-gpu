//! # Memory Layout
//! 
//! Describes the memory layout
//! a buffer should use
//! 

/// Requirement for being able to mark
/// other structs with memory information
pub trait MemSeal {}

/// Contiguous blocks of
/// memory, represented as
/// [[x, x], [y, y]]
/// 
/// Should be used if you
/// want to perform operations
/// on each axis seperately
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SoA;

impl MemSeal for SoA {}

/// Interleaved blocks of
/// memory, represented as
/// [[x, y], [x, y]]
/// 
/// Should be used if you
/// want to perform operations
/// on each axis together
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AoS;

impl MemSeal for AoS {}

/// I'm too lazy to write `ory` everytime
pub type MemLayout = MemoryLayout;

/// The memory layout a buffer should use
/// 
/// Different layouts have different
/// efficiency gains for different
/// use cases
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryLayout {
    /// Contiguous blocks of
    /// memory, represented as
    /// [[x, x], [y, y]]
    /// 
    /// Should be used if you
    /// want to perform operations
    /// on each axis seperately
    SoA,

    /// Interleaved blocks of
    /// memory, represented as
    /// [[x, y], [x, y]]
    /// 
    /// Should be used if you
    /// want to perform operations
    /// on each axis together
    #[default]
    AoS,
}
