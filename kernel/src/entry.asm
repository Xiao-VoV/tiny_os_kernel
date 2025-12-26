# kernel/src/entry.asm

    .section .text.entry
    .global _start
_start:
    # 1. 设置栈指针
    # la 是 load address，将 boot_stack_top 的地址加载到 sp 寄存器
    la sp, boot_stack_top

    # 2. 跳转到 Rust 代码
    # call 伪指令会将返回地址压栈，但这里我们不打算返回
    call rust_main

    # 3. 如果 rust_main 返回了（理论上不应该），我们进入死循环
    j .

    # 定义栈空间
    .section .bss.stack
    .global boot_stack_lower_bound
boot_stack_lower_bound:
    # 分配 4096 * 16 字节 (64KB) 的栈空间
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
    # 栈顶在这里（高地址）