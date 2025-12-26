# Rust Tint OS

[中文](#中文) | [English](#english)

---

<h2 id="中文">中文介绍</h2>

**Rust Tint OS** 是一个基于 Rust 语言编写的 RISC-V 操作系统内核。该项目旨在通过从零开始构建内核，深入学习操作系统底层原理，如内存管理、并发控制和设备驱动等。

### 主要特性

*   **架构**: RISC-V 64位 (RV64GC)
*   **语言**: Rust (no_std, 裸机环境)
*   **内存管理**:
    *   物理页帧分配 (Frame Allocator)
    *   堆内存分配 (Heap Allocator)
    *   虚拟内存与分页机制 (Paging & Page Tables)
*   **输出**: 基于 SBI 的控制台输出

### 项目结构

*   `kernel/`: 操作系统内核源码
*   `doc/`: 开发文档与笔记
*   `target/`: 编译产物

### 构建与运行

本项目使用 Cargo 进行管理。

```bash
cd kernel
cargo build --release
```
### 参考文献

> [rCore OS](https://github.com/rcore-os/rCore)  
> [rCore OS doc](https://rcore-os.cn/rCore-Tutorial-Book-v3/index.html)  
> [Writing an OS in Rust](https://os.phil-opp.com/)  
> [osdev wiki](https://wiki.osdev.org/Expanded_Main_Page)  


---

<h2 id="english">English Introduction</h2>

**Rust Tint OS** is an operating system kernel written in Rust for the RISC-V architecture. This project is designed as a learning journey to understand low-level operating system principles, including memory management, concurrency, and device drivers, by building a kernel from scratch.

### Key Features

*   **Architecture**: RISC-V 64-bit (RV64GC)
*   **Language**: Rust (no_std, Bare Metal)
*   **Memory Management**:
    *   Physical Frame Allocation
    *   Heap Allocation
    *   Virtual Memory & Paging
*   **Output**: SBI-based Console Output

### Project Structure

*   `kernel/`: Source code of the OS kernel
*   `doc/`: Documentation and development notes
*   `target/`: Build artifacts

### Build and Run

This project is managed by Cargo.

```bash
cd kernel
cargo build --release
```

### Reference

> [rCore OS](https://github.com/rcore-os/rCore)  
> [rCore OS doc](https://rcore-os.cn/rCore-Tutorial-Book-v3/index.html)  
> [Writing an OS in Rust](https://os.phil-opp.com/)  
> [osdev wiki](https://wiki.osdev.org/Expanded_Main_Page)  