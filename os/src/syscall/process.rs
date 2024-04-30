//! Process management syscalls
// use alloc::task;

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{memory_set::{mmap_allocal, mmap_drop}, page_table::get_ppt}, syscall::{SYSCALL_GET_TIME, SYSCALL_MMAP, SYSCALL_MUNMAP, SYSCALL_TASK_INFO, SYSCALL_YIELD}, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_init_time, get_systemcall_times, suspend_current_and_run_next, task_count, TaskStatus
    }, timer::{get_time_ms, get_time_us}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    task_count(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    task_count(SYSCALL_GET_TIME);
    let us: usize = get_time_us();
    let times = get_ppt(current_user_token(), _ts);
    *times = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    task_count(SYSCALL_TASK_INFO);
    
    // let cur_init_time = cur_init_time();
    // let get_time_ms = get_time_ms();
    let task_info = get_ppt(current_user_token(), _ti);
    task_info.status = TaskStatus::Running;
    task_info.syscall_times = get_systemcall_times();
    task_info.time = get_time_ms() - get_current_task_init_time();
    
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    task_count(SYSCALL_MMAP);
    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    mmap_allocal(_start, _len, _port)
    // 0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    task_count(SYSCALL_MUNMAP);
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    mmap_drop(_start, _len)
    // 0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
