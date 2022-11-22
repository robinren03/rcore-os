use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{block_current_and_run_next, current_process, current_task};
use crate::timer::{add_timer, get_time_ms};
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn sys_sleep(ms: usize) -> isize {
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}

// LAB5 HINT: you might need to maintain data structures used for deadlock detection
// during sys_mutex_* and sys_semaphore_* syscalls
pub fn sys_mutex_create(blocking: bool) -> isize {
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    if let Some(id) = process_inner
        .mutex_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        let mutex: Option<Arc<dyn Mutex>> = if !blocking {
            Some(Arc::new(MutexSpin::new(id)))
        } else {
            Some(Arc::new(MutexBlocking::new(id)))
        };
        process_inner.mutex_list[id] = mutex;
        let task_count = process_inner.tasks.len();
        for _i in 0..task_count {
            let task = process_inner.get_task(_i);
            let mut task_inner = task.inner_exclusive_access();
            task_inner.mutex_alloc[id] = 0;
            task_inner.mutex_need[id] = 0;
        }
        id as isize
    } else {
        let final_id = process_inner.mutex_list.len();
        let mutex: Option<Arc<dyn Mutex>> = if !blocking {
            Some(Arc::new(MutexSpin::new(final_id)))
        } else {
            Some(Arc::new(MutexBlocking::new(final_id)))
        };
        process_inner.mutex_list.push(mutex);
        let task_count = process_inner.tasks.len();
        for _i in 0..task_count {
            let task = process_inner.get_task(_i);
            let mut task_inner = task.inner_exclusive_access();
            task_inner.mutex_alloc.push(0);
            task_inner.mutex_need.push(0);
        }
        final_id as isize
    }
}

pub fn is_dead_mutex(detect: usize) -> bool {
    if detect == 0 {
        return false;
    }
    if detect != 1 {
        return true;
    } //error! Not supposed to have detect value like this
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    let task_count=inner.tasks.len();
    let mut work: Vec<usize> = Vec::new();
    let mut finish: Vec<bool> = Vec::new();

    for _i in 0..task_count {
        finish.push(false);
    }

    for i in 0..inner.mutex_list.len(){
        if let Some(mtx) = &mut inner.mutex_list[i]{
            if !mtx.is_locked(){
                work.push(1);
                continue;
            }
        }
        work.push(0);
    }
    
    loop {
        let mut exitable = true;
        let inner_tasks=&mut inner.tasks;
        for i in 0..task_count{
            if finish[i]{
                continue;
            }
            if let Some(task)=&mut inner_tasks[i]{
                let mut f=false;
                let task_inner = task.inner_exclusive_access();
                for j in 0..work.len(){
                    if task_inner.mutex_need[j] > work[j]{
                        f = true;
                        break;
                    }
                }
                if f {
                    continue;
                }
                exitable=false;
                finish[i]=true;

                for j in 0..work.len(){
                    work[j] += task_inner.mutex_alloc[j];
                }

                drop(task_inner);
            }
        }
        if exitable{
            break;
        }
    }
    for i in 0..task_count{
        if !finish[i]{
            return true;
        }
    }
    false
}

// LAB5 HINT: Return -0xDEAD if deadlock is detected
pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    let tem = process_inner.detect;
    drop(process_inner);
    drop(process);
    mutex.update();
    if is_dead_mutex(tem) {
        return -0xDEAD;
    }
    mutex.lock();
    0
}

pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    drop(process);
    mutex.unlock();
    0
}

pub fn sys_semaphore_create(res_count: usize) -> isize {
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .semaphore_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.semaphore_list[id] = Some(Arc::new(Semaphore::new(res_count, id)));
        let task_count = process_inner.tasks.len();
        for _i in 0..task_count {
            let task = process_inner.get_task(_i);
            let mut task_inner = task.inner_exclusive_access();
            task_inner.sem_alloc[id] = 0;
            task_inner.sem_need[id] = 0;
        }
        id as isize
    } else {
        let final_id = process_inner.semaphore_list.len();
        process_inner
            .semaphore_list
            .push(Some(Arc::new(Semaphore::new(res_count, final_id))));
        let task_count = process_inner.tasks.len();
        for _i in 0..task_count {
            let task = process_inner.get_task(_i);
            let mut task_inner = task.inner_exclusive_access();
            task_inner.sem_alloc.push(0);
            task_inner.sem_need.push(0);
        }
        final_id as isize
    };
    id as isize
}

pub fn sys_semaphore_up(sem_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    drop(process_inner);
    sem.up();
    0
}

pub fn is_dead_sem(detect: usize) -> bool {
    if detect == 0 {
        return false;
    }
    if detect != 1 {
        return true;
    } //error! Not supposed to have detect value like this
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    let task_count=inner.tasks.len();
    let mut work: Vec<usize> = Vec::new();
    let mut finish: Vec<bool> = Vec::new();

    for _i in 0..task_count {
        finish.push(false);
    }

    for i in 0..inner.semaphore_list.len(){
        if let Some(sem) = &mut inner.semaphore_list[i]{
            if sem.inner.exclusive_access().count > 0{
                work.push(sem.inner.exclusive_access().count as usize);
                continue;
            }
        }
        work.push(0);
    }
    
    loop {
        let mut exitable = true;
        let inner_tasks=&mut inner.tasks;
        for i in 0..task_count{
            if finish[i]{
                continue;
            }
            if let Some(task)=&mut inner_tasks[i]{
                let mut f=false;
                let task_inner = task.inner_exclusive_access();
                for j in 0..work.len(){
                    if task_inner.sem_need[j] > work[j]{
                        f = true;
                        break;
                    }
                }
                if f {
                    continue;
                }
                exitable=false;
                finish[i]=true;

                for j in 0..work.len(){
                    work[j] += task_inner.sem_alloc[j];
                }

                drop(task_inner);
            }
        }
        if exitable{
            break;
        }
    }
    for i in 0..task_count{
        if !finish[i]{
            return true;
        }
    }
    false
}

// LAB5 HINT: Return -0xDEAD if deadlock is detected
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    let tem = process_inner.detect;
    drop(process_inner);
    sem.update();
    if is_dead_sem(tem){
       return -0xDEAD;
    }
    sem.down();
    0
}

pub fn sys_condvar_create(_arg: usize) -> isize {
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .condvar_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.condvar_list[id] = Some(Arc::new(Condvar::new()));
        id
    } else {
        process_inner
            .condvar_list
            .push(Some(Arc::new(Condvar::new())));
        process_inner.condvar_list.len() - 1
    };
    id as isize
}

pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    drop(process_inner);
    condvar.signal();
    0
}

pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    condvar.wait(mutex);
    0
}

// LAB5 YOUR JOB: Implement deadlock detection, but might not all in this syscall
pub fn sys_enable_deadlock_detect(_enabled: usize) -> isize {
    if _enabled==0 ||_enabled==1{
        let process=current_process();
        let mut _inner=process.inner_exclusive_access();
        _inner.detect=_enabled;
        return 1;
    }
    -1
}
