//! # Adapter Description
//! 
//! An adapter description
//! 

use std::marker::PhantomData;

use crate::{adapter::Adapter, buffer::{AoSBuffer, Buffer, SoABuffer}, memory_layout::{AoS, IntoMemLayout, MemLayout, MemSeal, SoA}};

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

impl ADI for AdapterDescription<AoS> {
    type SwitchMem = SoA;
    type Buf = AoSBuffer;
}

impl IntoMemLayout for AdapterDescription<AoS> {
    fn get_memory_layout(&self) -> MemLayout {
        MemLayout::AoS
    }
}

impl ADI for AdapterDescription<SoA> {
    type SwitchMem = AoS;
    type Buf = SoABuffer;
}

impl IntoMemLayout for AdapterDescription<SoA> {
    fn get_memory_layout(&self) -> MemLayout {
        MemLayout::SoA
    }
}

/// Adapter Description Implementation
pub trait ADI {
    type SwitchMem: MemSeal;
    type Buf: Buffer;

    /// Consumes itself, switches the memory layout, and returns a new copy
    fn switch_memory_layout(self) -> AdapterDescription<Self::SwitchMem>
    where
        Self: Sized,
    {
        AdapterDescription {
            _mem_mark: PhantomData,
        }
    }

    /// Creates an `Adapter` from the configuration
    fn build(&self) -> Adapter<Self::Buf>
    where
        for<'a> Adapter<Self::Buf>: From<&'a Self>,
    {
        Adapter::from(self)
    }
}
