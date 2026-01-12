.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # 此时，csrw sscratch, sp 已经执行过（在 __restore 结尾）
    # sscratch 保存着用户栈指针 (User SP)
    # sp (即 x2) 目前是用户栈指针
    csrrw sp, sscratch, sp
    # 现在：
    # sp -> 指向内核分配给该进程的 TrapContext (在 TRAMPOLINE 下方)
    # sscratch -> 用户栈指针

    # 1. 保存通用寄存器到 TrapContext
    # x0 (zero) 不用存，x1 (ra) 存到偏移 8 (1*8)
    sd x1, 1*8(sp)
    # x2 (sp) 需要存用户栈指针，它现在在 sscratch 里
    sd x3, 3*8(sp)
    # 宏循环保存 x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    
    # 手动保存 x2 (User SP) 和 x4 (tp)
    csrr t0, sscratch
    sd t0, 2*8(sp)
    sd x4, 4*8(sp)

    # 2. 保存 CSR 状态寄存器
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    # 3. 读取内核相关信息 (这些信息在 TrapContext 初始化时填入)
    # 偏移 34*8 -> kernel_satp (内核页表)
    # 偏移 35*8 -> kernel_sp (内核栈顶)
    # 偏移 36*8 -> trap_handler (内核处理函数入口)
    ld t0, 34*8(sp)
    ld t1, 35*8(sp)
    ld t2, 36*8(sp)

    # 4. 切换到内核地址空间
    # 此时 sp 仍然指向 User Address Space 的 TrapContext (虚拟地址 TRAMPOLINE - PAGE_SIZE)
    # 切换 satp 后，因为 TrapContext 的页面和 Trampoline 代码页在内核页表中也被映射到了相同位置
    # 所以指令流不会中断（平滑过渡）
    csrw satp, t0
    sfence.vma
    
    # 5. 切换栈并跳转
    mv sp, t1      # 切换到内核栈
    jr t2          # 跳转到 trap_handler

__restore:
    # 此时 a0: TrapContext 指针 (User Address Space VA)
    # a1: User Space Token (satp value)
    
    # 1. 切换页表回用户空间
    # 注意：这里我们先切换 satp，再恢复寄存器。
    # 因为 a0 指向的 TrapContext 虚拟地址在用户页表中才有效（虽然在 Trampoline 区域是共享映射，但逻辑上属于用户）
    # 实际上由于 trampoline 区域被双重映射，在内核态也可以访问，但使用用户页表更符合语义
    csrw satp, a1
    sfence.vma
    
    # 2. 恢复 sp 到 TrapContext 位置
    # 此时 a0 是 TrapContext 在用户空间的位置 (TRAP_CONTEXT)
    csrw sscratch, a0
    mv sp, a0

    # 3. 恢复 CSR
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1

    # 4. 恢复通用寄存器
    # 先恢复除 sp, tp 以外的
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    
    # 恢复 x4 (tp)
    ld x4, 4*8(sp)

    # 5. 恢复用户栈指针 x2 (sp)
    # 最后一步：sp 此时还指向 TrapContext，我们需要获取其中保存的用户 sp
    ld sp, 2*8(sp)
    
    # 6. 返回用户态
    sret