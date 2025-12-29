pub mod context;
pub mod manager;
pub mod task_block;

use context::TaskContext;

use manager::suspend_current_and_run_next;
use manager::TASK_MANAGER;
use task_block::TaskControlBlock;
use task_block::TaskStatus;

const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 4;

#[repr(align(4096))]
#[derive(Debug, Clone, Copy)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Debug, Clone, Copy)]
struct KernelStackArray {
    stacks: [KernelStack; MAX_APP_NUM],
}

static mut KERNEL_STACK: KernelStackArray = KernelStackArray {
    stacks: [KernelStack {
        data: [0; KERNEL_STACK_SIZE],
    }; MAX_APP_NUM],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        (self.data.as_ptr() as usize) + KERNEL_STACK_SIZE //栈指针是从高往低正常的，所以指向的是最后
    }

    pub fn push_context(&self, cx: TaskContext) -> usize {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TaskContext>()) as *mut TaskContext;
        unsafe {
            *cx_ptr = cx;
        }
        cx_ptr as usize
    }
}

pub fn init() {
    let mut task_manager = TASK_MANAGER.lock();

    let sp_a = unsafe { KERNEL_STACK.stacks[0].get_sp() };
    let task_a = TaskControlBlock {
        task_cx: TaskContext::goto_restore(sp_a, task_a_entry as usize),
        task_status: TaskStatus::Ready,
    };
    task_manager.add_task(task_a);

    // 创建任务 B
    let sp_b = unsafe { KERNEL_STACK.stacks[1].get_sp() };
    let task_b = TaskControlBlock {
        task_status: TaskStatus::Ready,
        task_cx: TaskContext::goto_restore(sp_b, task_b_entry as usize),
    };
    task_manager.add_task(task_b);
}

// 任务 A 的入口
fn task_a_entry() {
    for i in 0..5 {
        println!("Task A: {}", i);
        suspend_current_and_run_next(); // 主动让出 CPU
    }
    println!("Task A finished!");
    // 任务结束后的处理比较麻烦（需要 exit），这里先死循环
    loop {
        suspend_current_and_run_next();
    }
}

// 任务 B 的入口
fn task_b_entry() {
    for i in 0..5 {
        println!("Task B: {}", i);
        suspend_current_and_run_next();
    }
    println!("Task B finished!");
    loop {
        suspend_current_and_run_next();
    }
}
