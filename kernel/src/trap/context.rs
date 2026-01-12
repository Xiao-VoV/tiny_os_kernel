use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub x: [usize; 32],      // 0-31: 通用寄存器 (x0-x31)
    pub sstatus: Sstatus,    // 32: Supervisor Status Register
    pub sepc: usize,         // 33: Supervisor Exception Program Counter
    pub kernel_satp: usize,  // 34: 内核页表物理页号
    pub kernel_sp: usize,    // 35: 内核栈指针
    pub trap_handler: usize, // 36: Trap 处理入口地址
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        unsafe {
            let mut sstatus = sstatus::read();
            sstatus.set_spp(SPP::User);
            sstatus.set_spie(true);

            let mut cx = Self {
                x: [0; 32],
                sstatus,
                sepc: entry,
                kernel_satp,
                kernel_sp,
                trap_handler,
            };

            cx.set_sp(sp);
            cx
        }
    }
}
