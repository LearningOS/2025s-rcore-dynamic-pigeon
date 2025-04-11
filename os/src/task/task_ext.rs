use crate::config::MAX_SYSCALL_NUM;


/// Task information
pub struct TaskInfo {
    /// Syscall numbers
    pub syscall_num: [isize; MAX_SYSCALL_NUM]
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            syscall_num: [0; MAX_SYSCALL_NUM],
        }
    }
}

/// Task priority and stride
#[derive(Debug, Clone, Copy)]
pub struct TaskPriority {
    /// The priority of the task
    pub priority: usize,
    /// The time slice of the task
    pub stride: usize,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self {
            priority: 16,
            stride: 0,
        }
    }
}

impl PartialEq for TaskPriority {
    fn eq(&self, other: &Self) -> bool {
        self.stride == other.stride
    }
}

impl Eq for TaskPriority {}

impl PartialOrd for TaskPriority {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.stride.partial_cmp(&other.stride)
    }
}

impl Ord for TaskPriority {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.stride.cmp(&other.stride)
    }
    
}

impl TaskPriority {
    const BIG_STRIDE: usize = 1_usize << 31;

    pub fn add_stride(&mut self) {
        self.stride += Self::BIG_STRIDE / self.priority;
    }

    pub fn set_priority(&mut self, priority: usize) {
        self.priority = priority;
    }
}