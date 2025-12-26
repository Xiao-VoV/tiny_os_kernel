#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(alloc_error_handler)]

mod sbi;
#[macro_use] // 导出 console 模块中的宏 (println!, print!)
mod console;
mod mm;

use core::arch::global_asm;
use core::panic::PanicInfo;

extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

// 引入汇编代码
// 这里的 include_str! 宏会将汇编文件作为字符串嵌入
global_asm!(include_str!("entry.asm"));

// 这个函数在 panic 发生时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 现在 panic 时我们可以打印错误信息了！
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    loop {}
}

// 注意：这里不再需要 #[link_section = ".text.entry"]
// 因为 entry.asm 中的 _start 已经标记为 .text.entry 了
// 也不需要 #[no_mangle] 了，因为我们在汇编里是用 rust_main 调用的
// 但是为了保险起见，避免编译器混淆名字，我们还是保留 no_mangle
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 清屏 (ANSI 转义序列)
    print!("\x1b[2J");
    // 光标移动到左上角
    print!("\x1b[H");

    println!("Hello, World!");
    println!("I am a Rust OS Kernel running on RISC-V!");

    // --- 内存分配
    mm::init();
    println!("end mm init");
    // 测试内存分配
    let frame1 = mm::frame_allocator::alloc_frame();
    let frame2 = mm::frame_allocator::alloc_frame();
    println!("Allocated frame 1: {:?}", frame1);
    println!("Allocated frame 2: {:?}", frame2);

    if let Some(f) = frame1 {
        mm::frame_allocator::dealloc_frame(f);
        println!("Deallocated frame 1");
    }

    let frame3 = mm::frame_allocator::alloc_frame();
    println!(
        "Allocated frame 3: {:?} (Should be same as frame 1)",
        frame3
    );
    // ---

    let b = Box::new(42);
    println!("Box value: {}", b);

    let mut v = Vec::new();
    for i in 0..5 {
        v.push(i);
    }
    println!("Vec: {:?}", v);

    panic!("Crash test!");
}
