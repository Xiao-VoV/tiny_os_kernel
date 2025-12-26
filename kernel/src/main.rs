#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::arch::global_asm;
use core::panic::PanicInfo;

// 引入汇编代码
// 这里的 include_str! 宏会将汇编文件作为字符串嵌入
global_asm!(include_str!("entry.asm"));

// 这个函数在 panic 发生时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 注意：这里不再需要 #[link_section = ".text.entry"]
// 因为 entry.asm 中的 _start 已经标记为 .text.entry 了
// 也不需要 #[no_mangle] 了，因为我们在汇编里是用 rust_main 调用的
// 但是为了保险起见，避免编译器混淆名字，我们还是保留 no_mangle
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 此时栈已经初始化好了，我们可以安全地使用 Rust 语言特性了

    // 这是一个死循环，防止函数返回
    loop {}
}
