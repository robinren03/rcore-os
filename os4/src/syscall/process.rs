//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::mm::{VirtAddr, MapPermission, get_easy_ptr_from_token};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, map, unmap, current_user_token, get_syscall_times, get_first_time};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let token = current_user_token();
    let ts: &mut TimeVal = get_easy_ptr_from_token(token, _ts as *const u8);
    *ts = TimeVal {
        sec: _us/1_000_000,
        usec: _us%1_000_000,
    };
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    let start_addr = VirtAddr::from(_start);
    let start_offset : usize = start_addr.page_offset();
    if start_offset>0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }

    let end_addr:VirtAddr = VirtAddr::from(_start + _len);
    let end_page_num = end_addr.ceil();
    let mut permission=MapPermission::U;
    if _port&0x1!=0 {
        permission |= MapPermission::R;
    }
    if _port&0x2!=0 {
        permission |= MapPermission::W;
    }
    if _port&0x4!=0 {
        permission |= MapPermission::X;
    }
    if !map(start_addr.floor(), end_page_num, permission){
        return -1;
    }
    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    let start_addr=VirtAddr::from(_start);
    let start_offset : usize = start_addr.page_offset();
    if start_offset>0 {
        return -1;
    }
    let end_addr=VirtAddr::from(_start+_len);
    let end_page_num=end_addr.ceil();
    if !unmap(start_addr.floor(),end_page_num){
        return -1;
    }
    0
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    let _us = get_time_us();
    let token = current_user_token();
    let ti: &mut TaskInfo = get_easy_ptr_from_token(token, _ti as *const u8);
    *ti = TaskInfo {
       status: TaskStatus::Running,
       syscall_times: get_syscall_times(),
       time: (_us - get_first_time()) / 1000
    };
    0
}
