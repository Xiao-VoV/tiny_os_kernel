pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_set;
pub mod page_table;

pub fn init() {
    println!("mm init");
    frame_allocator::init();
    heap_allocator::init_heap();

    // 初始化内核地址空间并激活分页！
    println!("Initializing kernel address space...");
    let kernel_memory_set = memory_set::MemorySet::new_kernel();
    kernel_memory_set.activate();
    println!("Paging enabled!");
}
