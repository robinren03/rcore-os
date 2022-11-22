# Lab5实验报告

任彦羽 2019011215

### 实现功能

使用改良的银行家算法实现检测死锁的功能。

在`TaskControlBlockInner`中加入`(mutex/sem)_(alloc/need)`向量，分别支持锁和信号量的死锁检查，向量中的元素随创建锁/信号量、开关锁、执行信号量P/V操作时相应变化。非主线程的新线程生成时，上述向量应当为和主线程（或者当前线程）长度一致的全零向量。

在`ProcessControlBlockInner`中加入死锁检查使能变量`detect`。

在` sys_enable_deadlock_detect`中，对`detect`进行修改。在上锁、信号量P操作时，根据死锁检查使能，使用实验指导书中说明的算法对锁或者信号量进行死锁检查。

本次实验大约用时10小时 。

### 问答题

1.在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 - 需要回收的资源有哪些？ - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

需要回收线程的用户态栈、用于系统调用和异常处理的跳板页、内核栈等资源。其他线程的 TaskControlBlock 在锁机制、信号量机制、条件变量机制的实现时可能被引用。需要回收。

2.对比以下两种 `Mutex.unlock` 的实现，二者有什么区别？这些区别可能会导致什么问题？

```rust
impl Mutex for Mutex1 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        mutex_inner.locked = false;
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        }
    }
}

impl Mutex for Mutex2 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}

```

Mutex1 先解除互斥锁，再唤醒等待线程，可能导致互斥锁解除后被其他线程使用导致互斥锁失效产生混乱。Mutex2 先唤醒等候线程，再解除互斥锁。后者可能导致等候线程被唤醒后互斥锁仍未解除。