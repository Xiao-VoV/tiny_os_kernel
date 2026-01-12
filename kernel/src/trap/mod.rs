pub mod context;

use core::arch::global_asm;
use riscv::register::sstatus::{self, Sstatus, SPP};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
}
