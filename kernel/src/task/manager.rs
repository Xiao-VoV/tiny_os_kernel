use super::context::TaskContext;
use super::task_block::{TaskControlBlock, TaskStatus};
use alloc::vec::Vec;
use core::arch::global_asm;
use lazy_static::lazy_static; // 需要引入 lazy_static 依赖

global_asm!(include_str!("switch.S"));

extern "C" {
    fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

pub struct TaskManager {
    inner: Vec<TaskControlBlock>,
    current_task: usize,
}

impl TaskManager {
    fn find_next_task_cx(&mut self) -> Option<(*mut TaskContext, *const TaskContext)> {
        if let Some(next) = self.find_next_task() {
            let current = self.current_task;
            self.inner[current].task_status = TaskStatus::Ready;
            self.inner[next].task_status = TaskStatus::Running;
            self.current_task = next;

            let current_task_cx_ptr = &mut self.inner[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &self.inner[next].task_cx as *const TaskContext;

            Some((current_task_cx_ptr, next_task_cx_ptr))
        } else {
            None
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let n = self.inner.len();
        for i in 1..=n {
            let next = (self.current_task + i) % n;
            if self.inner[next].task_status == TaskStatus::Ready {
                return Some(next);
            }
        }
        None
    }

    pub fn add_task(&mut self, task: TaskControlBlock) {
        self.inner.push(task);
    }
}

pub fn run_first_task() {
    let mut task_manager = TASK_MANAGER.lock();
    let task0 = &mut task_manager.inner[0];
    task0.task_status = TaskStatus::Running;
    let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
    drop(task_manager); // 释放锁

    let mut _unused = TaskContext::zero_init();
    unsafe {
        __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
    }
    panic!("unreachable in run_first_task!");
}

pub fn suspend_current_and_run_next() {
    // 1. 获取锁
    let mut task_manager = TASK_MANAGER.lock();
    // 2. 获取切换所需的指针
    let task_cx_ptr = task_manager.find_next_task_cx();
    // 3. 显式释放锁！
    drop(task_manager);

    // 4. 进行切换
    if let Some((current, next)) = task_cx_ptr {
        unsafe {
            __switch(current, next);
        }
    } else {
        crate::println!("All tasks completed!");
        loop {}
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: spin::Mutex<TaskManager> = spin::Mutex::new(TaskManager {
        inner: Vec::new(),
        current_task: 0,
    });
}
