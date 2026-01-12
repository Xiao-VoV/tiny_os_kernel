/* filepath: /home/ubuntuxp/OS/kernel/src/linker.ld */
OUTPUT_ARCH(riscv)
ENTRY(_start)

/* OpenSBI 通常加载内核到这个地址，预留 2MB 给 SBI */
BASE_ADDRESS = 0x80200000; 

SECTIONS
{
    /* . 代表当前地址 (Location Counter) */
    . = BASE_ADDRESS;

    /* 这里的 start 符号标记了内核代码的开始位置 */
    skernel = .;

    /* 代码段 (Text Segment) */
    stext = .;
    .text : {
        *(.text.entry) /* 确保入口函数放在最前面 */
        *(.text .text.*)
    }


    /* Trampoline 段: 必须按页对齐，以便可以用作 User 和 Kernel 共享的跳板页 */
    . = ALIGN(4096);
    .text.trampoline : {
        strampoline = .;
        *(.text.trampoline)
        . = ALIGN(4096);
    }

    etext = .;

    /* 只读数据段 (Read-Only Data) */
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    /* 数据段 (Data Segment) - 已初始化的数据 */
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;


    /* BSS 段 - 未初始化的数据 (通常会被清零) */
    sbss_with_stack = .;
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }
    . = ALIGN(4K);
    ebss = .;

    . = ALIGN(4096);
    /* 这里的 end 符号标记了内核代码的结束位置 */
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}