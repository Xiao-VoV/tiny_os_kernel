### 第二课：交叉编译环境配置

我们的开发机（Host）是 Linux (x86_64)，但我们要运行的目标机器（Target）是 RISC-V 64。这就需要**交叉编译**。

我们需要告诉 Rust 编译器：“请生成 RISC-V 的机器码，而不是 x86 的机器码。”

#### 1. 添加目标架构 (Target Triple)

我们需要添加 `riscv64gc-unknown-none-elf` 这个目标。
*   `riscv64`: 64位 RISC-V 架构。
*   `gc`: 代表 "General purpose" (IMAFDC) + "Compressed" 指令集，这是最通用的组合。
*   `unknown`: 厂商未知（我们是裸机）。
*   `none`: 无操作系统（Bare Metal）。
*   `elf`: 输出格式为 ELF。

在终端运行以下命令（如果还没有安装）：

```bash
rustup target add riscv64gc-unknown-none-elf
```

#### 2. 自动化构建配置 (工程化实践)

为了工程化，我们不希望每次编译都手动输入 `--target riscv64gc-unknown-none-elf`。我们可以利用 Cargo 的配置文件来固化这个设置。

在工作区根目录 (OS) 下创建一个 `.cargo` 文件夹，并在其中创建 `config.toml`。

**注意**：是 `/home/ubuntuxp/OS/.cargo/config.toml`，而不是在 kernel 目录下。这样未来如果添加其他 crate（比如驱动），它们也会自动继承这个配置。

````toml
[build]
target = "riscv64gc-unknown-none-elf"

[target.riscv64gc-unknown-none-elf]
# 这里我们配置链接器参数
# -C link-arg=-Tsrc/linker.ld: 指定链接脚本（我们马上就会创建）
# -C force-frame-pointers=yes: 强制生成帧指针，方便调试和栈回溯
rustflags = [
    "-C", "link-arg=-Tkernel/src/linker.ld", 
    "-C", "force-frame-pointers=yes"
]
````

#### 3. 内存布局与链接脚本 (Linker Script)

这是操作系统开发中最关键的一步。
普通的程序由操作系统加载，操作系统决定把它放在内存的哪里。但我们是内核，我们必须自己决定**“我该住在内存的哪里”**。
QEMU 的 RISC-V 虚拟机默认将物理内存映射到 `0x80000000`。我们需要编写一个链接脚本，告诉编译器把我们的代码放在这个地址开始的地方。

创建 `/home/ubuntuxp/OS/kernel/src/linker.ld`：

````ld
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

    /* 这里的 end 符号标记了内核代码的结束位置 */
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
````

#### 4. 微调入口点

为了配合链接脚本中的 `*(.text.entry)`，我们需要修改 main.rs，把 `_start` 函数放入 `.text.entry` 段中。这样可以确保 `_start` 始终位于二进制文件的最开始，也就是 `0x80200000` 的位置，保证 QEMU 跳转过来时能直接执行到第一条指令。

修改 main.rs：

````rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".text.entry"] // <--- 新增：指定链接段
pub extern "C" fn _start() -> ! {
    loop {}
}
````

### 尝试编译

现在，一切准备就绪。在终端运行：

```bash
cargo build
```

如果成功，你将在 `target/riscv64gc-unknown-none-elf/debug/` 下看到 kernel 可执行文件。

下一节我们将使用 QEMU 启动它，并学习如何通过汇编代码初始化栈，让 Rust 代码真正跑起来。
