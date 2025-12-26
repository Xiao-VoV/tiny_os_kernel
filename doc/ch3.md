### 第三课：启动与栈初始化 (Bootstrapping)

虽然我们编译出了二进制文件，但如果现在直接运行，它会立即崩溃。为什么？

因为 **Rust 代码的执行依赖于栈 (Stack)**。
函数调用、局部变量都需要栈。但在裸机环境下，硬件上电时，栈指针寄存器 (`sp`) 的值是随机的（或者未定义的）。

我们需要写一小段 **汇编代码 (Assembly)** 来做两件事：
1.  分配一块内存作为启动栈。
2.  将 `sp` 寄存器指向这块内存的顶部（栈是向下增长的）。
3.  跳转到 Rust 代码。

#### 1. 编写汇编入口

我们需要创建一个汇编文件来接管入口点 `_start`。

创建文件 [`kernel/src/entry.asm`](kernel/src/entry.asm )：

```asm
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
```

#### 2. 嵌入汇编到 Rust

现在我们需要在 Rust 中引入这段汇编，并将之前的 `_start` 改名为 `rust_main`，以便汇编代码调用。

请修改 [`kernel/src/main.rs`](kernel/src/main.rs )：

```rust
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// 引入汇编代码
// 这里的 include_str! 宏会将汇编文件作为字符串嵌入
global_asm!(include_str!("entry.asm"));

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
```

#### 3. 配置 QEMU 运行器 (工程化实践)

为了方便调试，我们希望输入 `cargo run` 就能自动启动 QEMU。我们可以利用 Cargo 的 `runner` 配置。

修改 [`/.cargo/config.toml`](.cargo/config.toml )，在 `[target.riscv64gc-unknown-none-elf]` 部分添加 `runner`：

```toml
[build]
target = "riscv64gc-unknown-none-elf"

[target.riscv64gc-unknown-none-elf]
rustflags = [
    "-C", "link-arg=-Tkernel/src/linker.ld", 
    "-C", "force-frame-pointers=yes"
]
# 新增 runner 配置
# -machine virt: 使用 QEMU 的 virt 虚拟开发板
# -nographic: 不使用图形界面，直接在终端输出
# -bios default: 使用默认的 OpenSBI (Supervisor Binary Interface)
# -device loader,file=: 加载我们的内核文件
# -device loader,addr=0x80200000: 加载到指定地址
runner = "qemu-system-riscv64 -machine virt -nographic -bios default -device loader,file="
```

**注意**：上面的 `runner` 配置利用了 Cargo 的特性，Cargo 会把编译出来的可执行文件路径追加到命令后面。但是 `qemu` 的参数解析比较严格，直接追加可能无法正确识别为 `-kernel` 参数（或者我们需要用 `-device loader` 这种更通用的方式）。

为了更稳健，我们可以使用 `-kernel` 参数，但 Cargo 默认是把文件路径放在最后。我们可以写一个小脚本，或者使用更简单的参数形式。

我们尝试一个更简单的 `runner` 配置，利用 QEMU 的特性：

```toml
runner = "qemu-system-riscv64 -machine virt -nographic -bios default -kernel"
```

这样 `cargo run` 执行时，实际命令会变成 `qemu-system-riscv64 ... -kernel target/.../kernel`，这正是我们想要的。

### 验证时刻

现在，请运行：

```bash
cargo run
```

如果未安装qemu可以尝试使用以下命令进行安装：
- linux:
    - Arch: `pacman -S qemu`
    - Debian/Ubuntu:
      - For full `system emulation: apt-get install qemu-system`
      - For emulating Linux binaries: `apt-get install qemu-user-static`
    - Fedora: `dnf install @virtualization`
    - Gentoo: `emerge --ask app-emulation/qemu`
    - RHEL/CentOS: `yum install qemu-kvm`
    - SUSE: `zypper install qemu`
- macos
  - `brew install qemu`
- windows
  - 下载安装[QEMU WINDOWS](https://www.qemu.org/download/#windows)


**预期结果**：
你应该会看到 QEMU 启动，打印出 OpenSBI 的启动 Logo（一只 ASCII 艺术的恐龙？或者 OpenSBI 字样），然后停住不动。
这是正常的！因为我们的 `rust_main` 里只有一个 `loop {}`，而且我们还没有实现任何打印功能，所以它只是在默默地空转。