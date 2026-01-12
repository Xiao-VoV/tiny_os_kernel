use crate::mm::{
    address::{PhysPageNum, VirtPageNum},
    frame_allocator::alloc_frame,
};
use alloc::vec;
use alloc::vec::Vec;
use bitflags::bitflags;

// 在 RISC-V 的 SV39 分页模式下，一个页表项（PTE）是一个 64位 的整数，
// 它的每一位布局如下。
// [63:54]: 保留位 (Reserved)，必须为 0。
// [53:10]: 物理页号 (PPN)。共 44 位。这是映射到的物理地址的高位部分。
// [9:8]: 软件保留位 (RSW)，硬件忽略，操作系统可以用它存自定义信息。
// [7:0]: 标志位 (Flags)。
// V (Valid): 该项是否有效。如果为 0，访问会触发异常。
// R (Read) / W (Write) / X (Execute): 读/写/执行权限。
// U (User): 用户态是否可访问。
// G (Global): 全局映射（通常用于内核部分，TLB 刷新时不清除）。
// A (Accessed): CPU 访问过该页时会自动置 1。
// D (Dirty): CPU 写入过该页时会自动置 1。

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
// 页表项
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    /// 创建一个新的页表项，将物理页号 (PPN) 和标志位 (Flags) 组合成一个 64 位的整数
    pub fn new(physic_page_nnumber: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            // PPN 占据 [53:10]，所以需要左移 10 位
            // Flags 占据 [7:0]，直接按位或上去
            bits: (physic_page_nnumber.0 << 10 | flags.bits() as usize),
        }
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    /// 从页表项中提取物理页号 (PPN)
    pub fn ppn(&self) -> PhysPageNum {
        // 1. 右移 10 位，把 Flags 和 RSW 移出去
        // 2. 使用掩码 ((1 << 44) - 1) 取低 44 位
        //    (1 << 44) - 1 相当于 44 个 1 (0x0FFF_FFFF_FFFF)
        PhysPageNum((self.bits >> 10) & ((1usize << 44) - 1))
    }
    /// 从页表项中提取标志位 取低 8 位，转换为 PTEFlags 类型
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    /// 快速检查 V (Valid) 位 只有 V=1 时，CPU 才会认为这个映射是有效的
    pub fn is_valid(&self) -> bool {
        !(self.flags() & PTEFlags::V).is_empty()
    }
}

// 页表结构体

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<PhysPageNum>,
}
impl PageTable {
    pub fn new() -> Self {
        let frame = alloc_frame().expect("No frames for page table");
        PageTable {
            root_ppn: frame,
            frames: vec![frame],
        }
    }

    /// 获取根页表的物理页号（用于写入 satp 寄存器）
    pub fn root_ppn(&self) -> PhysPageNum {
        self.root_ppn
    }

    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    fn find_pte(&mut self, vpn: VirtPageNum, create: bool) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let result: Option<&mut PageTableEntry> = None;

        for (i, &idx) in idxs.iter().enumerate() {
            let pte_array = ppn.get_pte_array();
            let pte = &mut pte_array[idx];

            if i == 2 {
                return Some(pte);
            }

            if !pte.is_valid() {
                if !create {
                    return None;
                }
                let frame = alloc_frame()?;
                *pte = PageTableEntry::new(frame, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    // 建立映射
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte(vpn, true).expect("Map failed: no frames");
        if pte.is_valid() {
            panic!("VPN {:?} is already mapped", vpn);
        }
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    // 解除映射
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn, false).expect("Unmap failed: not mapped");
        if !pte.is_valid() {
            panic!("VPN {:?} is invalid", vpn);
        }
        *pte = PageTableEntry::empty();
    }

    /// 查找虚拟页号对应的物理页表项
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (i, &idx) in idxs.iter().enumerate() {
            let pte_array = ppn.get_pte_array();
            let pte = &pte_array[idx];

            if i == 2 {
                if pte.is_valid() {
                    return Some(*pte);
                } else {
                    return None;
                }
            }

            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        None
    }
}
