pub mod frame_allocator;
pub mod heap_allocator;

use crate::println;

pub fn init() {
    println!("mm init");
    frame_allocator::init();
    heap_allocator::init_heap();
}
