//! Adapter.rs
//! 
//! ... add docs here later
//! 

use crate::{adapter_desc::AdapterDescription, buffer::{AoSBuffer, Buffer, SoABuffer}, memory_layout::{AoS, IntoMemLayout, MemLayout, SoA}};

/// Add docs here later
#[derive(Debug)]
pub struct Adapter<B>
where
    B: Buffer,
{
    _buffer: B,
}

impl Default for Adapter<AoSBuffer> {
    fn default() -> Self {
        Self {
            _buffer: AoSBuffer::new(),
        }
    }
}

impl From<AdapterDescription<AoS>> for Adapter<AoSBuffer> {
    fn from(_value: AdapterDescription<AoS>) -> Self {
        Self::default()
    }
}

impl From<AdapterDescription<SoA>> for Adapter<SoABuffer> {
    fn from(_value: AdapterDescription<SoA>) -> Self {
        Self {
            _buffer: SoABuffer::new(),
        }
    }
}

impl From<&AdapterDescription<AoS>> for Adapter<AoSBuffer> {
    fn from(_value: &AdapterDescription<AoS>) -> Self {
        Self::default()
    }
}

impl From<&AdapterDescription<SoA>> for Adapter<SoABuffer> {
    fn from(_value: &AdapterDescription<SoA>) -> Self {
        Self {
            _buffer: SoABuffer::new(),
        }
    }
}

impl IntoMemLayout for Adapter<AoSBuffer> {
    fn get_memory_layout(&self) -> MemLayout {
        MemLayout::AoS
    }
}

impl IntoMemLayout for Adapter<SoABuffer> {
    fn get_memory_layout(&self) -> MemLayout {
        MemLayout::SoA
    }
}
