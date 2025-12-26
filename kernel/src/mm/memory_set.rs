use crate::mm::address::{PhysPageNum, VirtAddr, VirtPageNum};
use crate::mm::frame_allocator::{alloc_frame, dealloc_frame};
use crate::mm::page_table::{PTEFlags, PageTable};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use bitflags::bitflags;

// 映射类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapType {
    Identical, // 恒等映射 (虚拟地址 = 物理地址)，用于内核
    Framed,    // 帧映射 (分配新物理页)，用于用户程序/栈/堆
}

bitflags! {
    pub struct MapPermission:u8{
        const READ = 1 <<1;
        const WRITE = 1 <<2;
        const EXE = 1 <<3;
        const U = 1 <<4;
    }
}

pub struct MapArea {
    vpn_range: (VirtPageNum, VirtPageNum),           // [start,end)
    data_frames: BTreeMap<VirtPageNum, PhysPageNum>, //该区域使用的物理页 (仅 Framed 模式有效)
    map_type: MapType,
    map_permission: MapPermission,
}

impl MapArea {
    pub fn new(
        start_virtual_addr: VirtAddr,
        end_virtual_addr: VirtAddr,
        map_type: MapType,
        map_permission: MapPermission,
    ) -> Self {
        let start_virtual_page_number = start_virtual_addr.floor();
        let end_vpn = end_virtual_addr.ceil();
        Self {
            vpn_range: (start_virtual_page_number, end_vpn),
            data_frames: BTreeMap::new(),
            map_type,
            map_permission,
        }
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn_val in self.vpn_range.0 .0..self.vpn_range.1 .0 {
            let vpn = VirtPageNum(vpn_val);
            let ppn = match self.map_type {
                MapType::Identical => PhysPageNum(vpn_val),
                MapType::Framed => {
                    let frame = alloc_frame().expect("Out of merry!");
                    self.data_frames.insert(vpn, frame);
                    frame
                }
            };
            let pte_flags = PTEFlags::from_bits(self.map_permission.bits()).unwrap();
            page_table.map(vpn, ppn, pte_flags);
        }
    }
    #[allow(unused)]
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn_val in self.vpn_range.0 .0..self.vpn_range.1 .0 {
            let vpn = VirtPageNum(vpn_val);
            page_table.unmap(vpn);
            // 如果是 Framed，物理页会在 data_frames drop 时被释放（需要实现 Drop，或者手动释放）
            // 这里我们暂时手动处理：
            if self.map_type == MapType::Framed {
                if let Some(ppn) = self.data_frames.remove(&vpn) {
                    dealloc_frame(ppn);
                }
            }
        }
    }
}

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn push(&mut self, map_area: MapArea, _data: Option<&[u8]>) {
        self.areas.push(map_area);
    }

    // 修改：统一映射所有区域
    pub fn map_all_area(&mut self) {
        for area in self.areas.iter_mut() {
            area.map(&mut self.page_table);
        }
    }

    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        // 这里我们需要获取链接脚本中定义的各个段的地址
        // 使用 usize 获取地址
        let stext_addr = stext as *const () as usize;
        let etext_addr = etext as *const () as usize;
        let srodata_addr = srodata as *const () as usize;
        let erodata_addr = erodata as *const () as usize;
        let sdata_addr = sdata as *const () as usize;
        let edata_addr = edata as *const () as usize;
        let sbss_addr = sbss_with_stack as *const () as usize;
        let ebss_addr = ebss as *const () as usize;
        let ekernel_addr = ekernel as *const () as usize;

        println!("mapping .text section");
        memory_set.push(
            MapArea::new(
                VirtAddr(stext_addr),
                VirtAddr(etext_addr),
                MapType::Identical,
                MapPermission::READ | MapPermission::EXE,
            ),
            None,
        );

        println!("mapping .rodata section");
        memory_set.push(
            MapArea::new(
                VirtAddr(srodata_addr),
                VirtAddr(erodata_addr),
                MapType::Identical,
                MapPermission::READ,
            ),
            None,
        );

        println!("mapping .data section");
        memory_set.push(
            MapArea::new(
                VirtAddr(sdata_addr),
                VirtAddr(edata_addr),
                MapType::Identical,
                MapPermission::READ | MapPermission::WRITE,
            ),
            None,
        );

        println!("mapping .bss section");
        memory_set.push(
            MapArea::new(
                VirtAddr(sbss_addr),
                VirtAddr(ebss_addr),
                MapType::Identical,
                MapPermission::READ | MapPermission::WRITE,
            ),
            None,
        );

        println!("mapping physical memory");
        // 映射剩余的物理内存（包括堆、分配器管理的空闲页）
        // 简单起见，我们映射到内存的高地址，或者直接映射整个可用物理内存
        // 这里我们映射 ekernel 到 物理内存结束（假设 128MB 内存）
        // 注意：RISC-V QEMU virt 默认内存大小是 128MB，从 0x80000000 开始
        // 结束地址大约是 0x88000000
        memory_set.push(
            MapArea::new(
                VirtAddr(ekernel_addr),
                VirtAddr(0x88000000), // 硬编码一个足够大的结束地址
                MapType::Identical,
                MapPermission::READ | MapPermission::WRITE,
            ),
            None,
        );

        memory_set.map_all_area();
        memory_set
    }

    // 激活页表
    pub fn activate(&self) {
        let satp = self.page_table.root_ppn().0;
        // 模式 8 代表 SV39
        let satp_val = 8usize << 60 | satp;
        unsafe {
            // 写入 satp 寄存器
            core::arch::asm!("csrw satp, {}", in(reg) satp_val);
            // 刷新 TLB
            core::arch::asm!("sfence.vma");
        }
    }
}
