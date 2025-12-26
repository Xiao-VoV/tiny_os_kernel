use crate::mm::address::{PhysPageNum, PAGE_SIZE};

// 简单的栈式分配器
// 注意：这里我们硬编码了一个大小，实际 OS 中应该动态管理
// 假设我们最多管理 1000 个物理页 (约 4MB 内存)
const MAX_PHYSICAL_PAGES: usize = 1000;
struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: [usize; MAX_PHYSICAL_PAGES],
    recycled_count: usize,
}

static mut FRAME_ALLOCATOR: StackFrameAllocator = StackFrameAllocator {
    current: 0,
    end: 0,
    recycled: [0; MAX_PHYSICAL_PAGES],
    recycled_count: 0,
};

impl StackFrameAllocator {
    fn init(&mut self, start: PhysPageNum, end: PhysPageNum) {
        self.current = start.0;
        self.end = end.0;
        println!("Memory Area: [{:#x}, {:#x})", start.0, end.0);
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        // 1. 优先使用回收的页
        if self.recycled_count > 0 {
            self.recycled_count -= 1;
            return Some(PhysPageNum(self.recycled[self.recycled_count]));
        }
        // 2. 使用新的页
        if self.current == self.end {
            return None; // 内存耗尽
        }
        let p = self.current;
        self.current += 1;
        Some(PhysPageNum(p))
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        if self.recycled_count >= MAX_PHYSICAL_PAGES {
            panic!("Frame allocator recycled stack overflow!");
        }
        self.recycled[self.recycled_count] = ppn.0;
        self.recycled_count += 1;
    }
}

// 链接脚本中定义的符号
extern "C" {
    // 使用 static 声明，配合 & 取地址
    static ekernel: usize;
}

pub fn init() {
    println!("{}:{} start frame_allocatoe init!", file!(), line!());
    let ekernel_addr = unsafe { &ekernel as *const _ as usize };
    println!("{} {}: ekernel_addr {}", file!(), line!(), ekernel_addr);
    // 向上取整到页边界
    let start_pa = (ekernel_addr + PAGE_SIZE - 1) / PAGE_SIZE;

    let end_pa = start_pa + 1000;

    unsafe {
        FRAME_ALLOCATOR.init(PhysPageNum(start_pa), PhysPageNum(end_pa));
    }
}

pub fn alloc_frame() -> Option<PhysPageNum> {
    unsafe { FRAME_ALLOCATOR.alloc() }
}

pub fn dealloc_frame(ppn: PhysPageNum) {
    unsafe { FRAME_ALLOCATOR.dealloc(ppn) }
}
