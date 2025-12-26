use crate::mm::page_table::PageTableEntry;
// 物理地址相关结构体
// 物理页号
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysPageNum(pub usize);

// 物理地址
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysAddr(pub usize);

// 虚拟地址相关结构体
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtPageNum(pub usize);

// --- 常量 ---
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;

// --- 转换实现 ---
impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}
impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}
impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v)
    }
}
// PhysPageNum -> PhysAddr
impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        PhysAddr(v.0 << PAGE_SIZE_BITS)
    }
}
// PhysAddr -> PhysPageNum (默认向下取整)
impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.0 % PAGE_SIZE, 0); // 确保地址对齐
        PhysPageNum(v.0 / PAGE_SIZE)
    }
}
// VirtPageNum -> VirtAddr
impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> Self {
        VirtAddr(v.0 << PAGE_SIZE_BITS)
    }
}

// VirtAddr -> VirtPageNum
impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.0 % PAGE_SIZE, 0);
        VirtPageNum(v.0 / PAGE_SIZE)
    }
}
// ------

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512) }
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut index = [0usize; 3];

        for i in (0..3).rev() {
            index[i] = vpn & 511; //取低9位
            vpn >>= 9;
        }
        index
    }
}
