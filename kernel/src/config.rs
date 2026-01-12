pub const USER_STACK_SIZE:usize = 4096 * 2;
pub const KERNEL_STACK_SIZE:usize =4096*2;
pub const KERNEL_HEAP_SIZE :usize = 0x30_0000;

pub const PAGE_SIZE:usize = 4096;
pub const PAGE_SIZE_BITS:usize = 12;


// 跳板页的虚拟地址 (Trampoline)
// 放置在虚拟地址空间的最高一个页面
pub const TRAMPOLINE:usize = usize::MAX -PAGE_SIZE +1;
// Trap 上下文 (TrapContext) 在用户地址空间的位置
// 放置在 Trampoline 页面之下
pub const TRAP_CONTEXT:usize = TRAMPOLINE -PAGE_SIZE;