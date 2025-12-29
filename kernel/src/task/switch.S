# kernel/src/task/switch.S

    .altmacro
    .macro    SAVE_SN n
    sd        s\n, (\n+2)*8(a0)
    .endm
    .macro    LOAD_SN n
    ld        s\n, (\n+2)*8(a1)
    .endm

    .section  .text
    .global   __switch
__switch:
# ---------------------------------------------------------------
# 函数原型: __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext)
# 参数 a0: 指向当前任务上下文结构体 (TaskContext) 的指针
# 参数 a1: 指向下一个任务上下文结构体 (TaskContext) 的指针
# ---------------------------------------------------------------

# === 阶段 1: 保存当前现场 (Save Context) ===
# 此时 sp 指向当前任务的栈（如果是第一次调用，sp 指向 Boot Stack）

# 1. 保存返回地址 (ra) 到当前任务的上下文结构体中
# 这样下次切换回来时，ret 指令会从这里继续执行
    sd        ra, 0(a0)

# 2. 保存 Callee-Saved 寄存器 (s0-s11)
    .set      n, 0
    .rept     12
    SAVE_SN   %n
    .set      n, n + 1
    .endr

# 3. 关键点：保存当前的栈指针 (sp)！
# 这相当于把当前任务的栈“冻结”在了上下文结构体里
    sd        sp, 8(a0)

# === 阶段 2: 恢复下个现场 (Restore Context) ===

# 4. 恢复下一个任务的返回地址 (ra)
# 对于新任务，这个 ra 是我们在 goto_restore 中设置的入口地址 (如 task_a_entry)
    ld        ra, 0(a1)

# 5. 恢复 Callee-Saved 寄存器 (s0-s11)
    .set      n, 0
    .rept     12
    LOAD_SN   %n
    .set      n, n + 1
    .endr

# 6. 关键点：加载下一个任务的栈指针 (sp)！
# 这一步执行完，CPU 的栈就瞬间切换到了 KernelStackArray 中的某一块内存
# 从此以后，所有的 push/pop 操作都在新任务的栈上进行
    ld        sp, 8(a1)

# === 阶段 3: 执行流切换 ===

# 7. 跳转到 ra 指向的地址
# 如果切回老任务，ra 指向 __switch 调用后的下一条指令
# 如果切到新任务，ra 指向 task_a_entry
    ret