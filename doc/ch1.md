# 开篇

### 第一课：剥离操作系统的依赖 (Freestanding)
要编写操作系统内核，我们面临的第一个悖论是：**标准库 (`std`) 依赖于操作系统**（例如 `std::fs` 需要文件系统，`std::thread` 需要线程调度）。既然我们要写的是操作系统本身，我们就不能使用标准库。

我们需要进入 **裸机 (Bare Metal)** 模式。

#### 1. 核心概念
*   **`#![no_std]`**: 告诉 Rust 编译器不要链接标准库，只链接核心库 (`core`)。核心库是与架构无关的、不依赖操作系统的基础库（包含基本类型、切片、迭代器等）。
*   **`#![no_main]`**: 标准的 Rust 程序从 `main` 开始，但实际上在 `main` 之前，C 运行时 (crt0) 会进行初始化。内核没有运行时，所以我们需要覆盖入口点。
*   **Panic Handler**: 在标准库中，程序崩溃 (Panic) 时会打印错误并退出。在内核中，我们需要自己定义“崩溃”时该做什么（比如死循环挂起）。

#### 2. 动手实践

修改 main.rs，将其变为一个最小的内核骨架。

````rust
#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

// 这个函数在 panic 发生时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 这是一个裸函数，它是链接器寻找的入口点
// "C" 告诉编译器使用 C 调用约定
// _start 是大多数系统默认的入口点名称
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 这里是内核的起点
    // 目前我们什么都做不了，只能死循环
    loop {}
}
````

#### 3. 配置 Cargo

为了减小内核二进制文件的大小并避免处理复杂的栈展开（Stack Unwinding），我们需要配置 release 模式下的 panic 行为为 "abort"（直接终止）。

修改 Cargo.toml：

````toml
[package]
name = "kernel"
version = "0.1.0"
edition = "2021" # 建议使用 2021 或更新

# ...existing code...

# 添加以下配置
[profile.release]
panic = "abort"

[profile.dev]
panic = "abort"
````

### 这里的“模块化”思考

虽然现在只有几行代码，但我们也要尽量将代码解耦模块化方便未来工作：

1.  **Workspace 结构**: 我们采用Rust workspace工程，在根目录使用了 Cargo.toml 的 `[workspace]`。未来我们可以把 `drivers`（驱动）、`fs`（文件系统）拆分成独立的 crate 放在 workspace 中，kernel crate 只负责把它们组装起来。
2.  **解耦**: `panic_handler` 目前在 main.rs 中，未来我们可以将其移动到一个专门的 `logging` 或 `utils` 模块中，甚至根据不同的架构（RISC-V vs x86）有不同的实现。

### 下一步建议

现在的代码虽然能编译（通过 `cargo build --target <target>`），但还不能运行，因为我们还没有指定目标架构。

为了深入理解底层，下一阶段我们选择 **RISC-V 64** 架构。
*   **理由**: 指令集简洁、现代，没有 x86 那么重的历史包袱，非常适合教学和理解原理。
*   **工具**: 我们将使用 QEMU 模拟器来运行它。