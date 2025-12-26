pub mod frame_allocator;
use crate::println;

pub fn init() {
    println!("mm init");
    frame_allocator::init();
}
