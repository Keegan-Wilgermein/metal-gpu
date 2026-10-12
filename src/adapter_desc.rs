//! # Adapter Description
//! 
//! A adapter description
//! 

use std::marker::PhantomData;

use crate::{adapter::Adapter, buffer::{AoSBuffer, SoABuffer}, memory_layout::{AoS, MemLayout, MemSeal, SoA}};

/// Describes the adapter
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdapterDescription<M>
where
    M: MemSeal,
{
    _mem_mark: PhantomData<M>,
}

impl Default for AdapterDescription<AoS> {
    fn default() -> Self {
        Self {
            _mem_mark: PhantomData,
        }
    }
}

impl AdapterDescription<AoS> {
    /// Gets the memory layout
    pub fn get_memory_layout(&self) -> MemLayout {
        MemLayout::AoS
    }

    /// Consumes itself, switches the memory layout, and returns a new copy
    pub fn with_soa_layout(self) -> AdapterDescription<SoA> {
        AdapterDescription {
            _mem_mark: PhantomData,
        }
    }

    /// Creates an `Adapter` from the configuration
    pub fn build(&self) -> Adapter<AoSBuffer> {
        Adapter::from(self)
    }
}

impl AdapterDescription<SoA> {
    /// Gets the memory layout
    pub fn get_memory_layout(&self) -> MemLayout {
        MemLayout::SoA
    }

    /// Consumes itself, switches the memory layout, and returns a new copy
    pub fn with_aos_layout(self) -> AdapterDescription<AoS> {
        AdapterDescription {
            _mem_mark: PhantomData,
        }
    }

    /// Creates an `Adapter` from the configuration
    pub fn build(&self) -> Adapter<SoABuffer> {
        Adapter::from(self)
    }
}
