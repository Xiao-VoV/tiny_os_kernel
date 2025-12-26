use crate::mm::address::{PhysAddr, PhysPageNum};
use bitflags::bitflags;

bitflags! {
    pub struct PTEFlags:u8{
        const V = 1 << 0; // Valid
        const R = 1 << 1; // Read
        const W = 1 << 2; // Write
        const X = 1 << 3; // Execute
        const U = 1 << 4; // User
        const G = 1 << 5; // Global
        const A = 1 << 6; // Accessed
        const D = 1 << 7; // Dirty
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(physic_page_nnumber: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: (physic_page_nnumber.0 << 10 | flags.bits() as usize),
        }
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum((self.bits >> 10) & ((1usize << 44) - 1))
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        !(self.flags() & PTEFlags::V).is_empty()
    }
}
