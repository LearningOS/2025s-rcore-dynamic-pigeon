//! Types related to task management

use crate::sync::UPSafeCell;

use super::TaskContext;

/// The task control block (TCB) of a task.
// #[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The system call times
    pub syscall_times: SysCallTimes,
}

/// The system call times

pub struct SysCallTimes {
    /// The system call times
    pub syscall_times: UPSafeCell<[isize; 412]>,
}

impl SysCallTimes {
    /// Increase the system call times
    pub fn add_syscall_times(&self, syscall_id: usize) {
        self.syscall_times.exclusive_access()[syscall_id] += 1;
    }

    /// Get the system call times
    pub fn get_syscall_times(&self, syscall_id: usize) -> isize {
        self.syscall_times.exclusive_access()[syscall_id]
    }
}

impl Default for SysCallTimes {
    fn default() -> Self {
        SysCallTimes {
            syscall_times: unsafe { UPSafeCell::new([0; 412]) },
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
