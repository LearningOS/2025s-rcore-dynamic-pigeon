//! Process management syscalls
use crate::{
    config::PAGE_SIZE,
    mm::{translated_byte_buffer, MapPermission, VirtAddr},
    task::{
        change_program_brk, current_task, current_task_memory_set, current_user_token,
        exit_current_and_run_next, suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let mut time = &time as *const TimeVal as *const u8;

    let buffers = translated_byte_buffer(
        current_user_token(),
        ts as *const _,
        core::mem::size_of::<TimeVal>(),
    );

    for buffer in buffers {
        buffer.copy_from_slice(unsafe { core::slice::from_raw_parts(time, buffer.len()) });
        unsafe {
            time = time.add(buffer.len());
        }
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let task = current_task();
            if task
                .memory_set
                .check_flag(VirtAddr::from(id), MapPermission::R)
            {
                let buffers = translated_byte_buffer(
                    current_user_token(),
                    id as *const _,
                    core::mem::size_of::<u8>(),
                );
                buffers[0][0] as isize
            } else {
                -1
            }
        }
        1 => {
            let task = current_task();
            if task
                .memory_set
                .check_flag(VirtAddr::from(id), MapPermission::W)
            {
                let mut buffers = translated_byte_buffer(
                    current_user_token(),
                    id as *const _,
                    core::mem::size_of::<u8>(),
                );
                buffers[0][0] = data as u8;
                0
            } else {
                -1
            }
        }
        2 => {
            let task = current_task();
            task.task_info.get_syscall_time(id)
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if start & (PAGE_SIZE - 1) != 0 {
        return -1;
    }

    if port & 0x7 == 0 || port & !0x7 != 0 {
        return -1;
    }

    let map_permmision = MapPermission::from_bits_truncate(((port as u8) << 1) | (1 << 4));

    let end = VirtAddr::from((start + len + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE);
    let start = VirtAddr::from(start);

    let memory_set = current_task_memory_set();
    memory_set.try_insert_framed_area(start, end, map_permmision)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if start & (PAGE_SIZE - 1) != 0 {
        return -1;
    }
    let memset = current_task_memory_set();
    memset.try_remove_area(VirtAddr::from(start), VirtAddr::from(start + len))
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
