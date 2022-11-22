use super::UPSafeCell;
use crate::task::TaskControlBlock;
use crate::task::{add_task, current_task};
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use alloc::{collections::VecDeque, sync::Arc};

pub trait Mutex: Sync + Send {
    fn lock(&self);
    fn unlock(&self);
    fn is_locked(&self) -> bool;
    fn update(&self);
}

pub struct MutexSpin {
    locked: UPSafeCell<bool>,
    id: usize
}

impl MutexSpin {
    pub fn new(_id: usize) -> Self {
        Self {
            locked: unsafe { UPSafeCell::new(false) },
            id: _id
        }
    }
}

impl Mutex for MutexSpin {
    fn lock(&self) {
        loop {
            let mut locked = self.locked.exclusive_access();
            if *locked {
                drop(locked);
                suspend_current_and_run_next();
                continue;
            } else {
                let cur_task =current_task().unwrap();
                let mut current_task_inner = cur_task.inner_exclusive_access();
                current_task_inner.mutex_alloc[self.id]=1;
                current_task_inner.mutex_need[self.id]=0;
                drop(current_task_inner);
                *locked = true;
                return;
            }
        }
    }

    fn unlock(&self) {
        let mut locked = self.locked.exclusive_access();
        let cur_task=current_task().unwrap();
        let mut current_task_inner = cur_task.inner_exclusive_access();
        current_task_inner.mutex_alloc[self.id]=0;
        drop(current_task_inner);
        *locked = false;
    }

    fn is_locked(&self) -> bool {
       let locked = self.locked.exclusive_access();
       *locked
    }

    fn update(&self){
        let locked=self.locked.exclusive_access();
        let current_task=current_task().unwrap();
        if *locked
        {
            current_task.inner_exclusive_access().mutex_need[self.id]=1;
        }
        else {
            current_task.inner_exclusive_access().mutex_alloc[self.id]=1;
            current_task.inner_exclusive_access().mutex_need[self.id]=0;
        }
    }
}

pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
    id: usize
}

pub struct MutexBlockingInner {
    locked: bool,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    pub fn new(_id: usize) -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    wait_queue: VecDeque::new(),
                })
            },
            id: _id
        }
    }
}

impl Mutex for MutexBlocking {
    fn lock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            mutex_inner.locked = true;
        }
    }

    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        let current_task=current_task().unwrap();
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            waking_task.inner_exclusive_access().mutex_need[self.id] = 0;
            waking_task.inner_exclusive_access().mutex_alloc[self.id] = 1;
            current_task.inner_exclusive_access().mutex_alloc[self.id] = 0;
            add_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }

    fn is_locked(&self) -> bool {
        self.inner.exclusive_access().locked
    }

    fn update(&self){
        let inner=self.inner.exclusive_access();
        let current_task=current_task().unwrap();
        if inner.locked
        {
            current_task.inner_exclusive_access().mutex_need[self.id]=1;
        }
        else {
            current_task.inner_exclusive_access().mutex_alloc[self.id]=1;
            current_task.inner_exclusive_access().mutex_need[self.id]=0;
        }
    }
}
