/* filepath: /home/ubuntuxp/OS/kernel/src/linker.ld */
OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000; /* OpenSBI 通常加载内核到这个地址，预留 2MB 给 SBI */

SECTIONS
{
    /* . 代表当前地址 (Location Counter) */
    . = BASE_ADDRESS;

    /* 这里的 start 符号标记了内核代码的开始位置 */
    skernel = .;

    /* 代码段 (Text Segment) */
    .text : {
        *(.text.entry) /* 确保入口函数放在最前面 */
        *(.text .text.*)
    }

    /* 只读数据段 (Read-Only Data) */
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    /* 数据段 (Data Segment) - 已初始化的数据 */
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    /* BSS 段 - 未初始化的数据 (通常会被清零) */
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }
    
    . = ALIGN(4096);
    /* 这里的 end 符号标记了内核代码的结束位置 */
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}