#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,      //返回地址
    sp: usize,      // 栈指针
    s: [usize; 12], // s0-s11 寄存器
}

impl TaskContext {
    pub fn goto_restore(kstack_ptr: usize, entry: usize) -> Self {
        Self {
            ra: entry,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }

    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
}
