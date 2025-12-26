#![allow(unused)]

use core::arch::asm;

// Legacy SBI Extension IDs
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_SHUTDOWN: usize = 8;

/// 调用 SBI 服务
/// which: 服务 ID (Extension ID)
/// arg0, arg1, arg2: 传递给 SBI 的参数
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall", // 触发环境调用异常，跳转到 OpenSBI
            in("a0") arg0, // 参数 0
            in("a1") arg1, // 参数 1
            in("a2") arg2, // 参数 2
            in("a7") which, // 服务 ID 放入 a7 寄存器
            lateout("a0") ret, // 返回值从 a0 寄存器读取
        )
    }
    ret
}

/// 向控制台输出一个字符
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// 从控制台读取一个字符
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

/// 关闭系统
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    loop {} // 理论上不会运行到这里
}
