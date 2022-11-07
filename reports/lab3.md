# Lab3实验报告
任彦羽 2019011215

### 实现功能

1. 根据新的`TaskControlBlock`设计调整系统调用，使用`current_task()`取代原有的从数组中取某一元素来实现前向兼容。
2. 对`fork`中新进程`memory_set` 采用从elf文件里读取产生而不是复制父进程的方式，并根据从elf中文件中读取到的该进程入口`entry_point`等产生新的`TrapContext`存储在相应高地址空间实现`spawn`功能。
3. 向 `TaskControlBlockInner` 中加入 `stride`、`pass`、`priority `字段，每次`fetch`时遍历所有进程取出`pass`最小者，每次开始执行时除了原先的可能对任务开始时间进行修改外还对`pass`进行修改。

-----------

### 问答题

stride 算法原理非常简单，但是有一个比较大的问题。例如两个stride = 10 的进程，使用 8bit 无符号整形储存 pass， p1.pass = 255, p2.pass = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

- 实际情况是轮到 p1 执行吗？为什么？

  ```
  不是，虽然 p2.stride+p2.pass=260，但由于260>256整数溢出所以 p2.pass 从计算机看来是4，反而变得更小。不仅如此，可以预料在接下来的很长一段时间片内都会是 p2.pass 更小，因为 p1.pass=255 是 8-bit 无符号整数中最大的数，所以 p2.pass 最多也就是与其相等，这样 p2 的执行时间就远远大于 p1 的执行时间，不满足公平性要求。
  ```

  

我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， **在不考虑溢出的情况下** , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 PASS_MAX – PASS_MIN <= BigStride / 2。

- 为什么？尝试简单说明（不要求严格证明）。

  ```
  因为 stride=BigStride/priority，而 priority>=2，所以 stride<=BigStride/2。那么，在不考虑溢出的情况下（或者我们认为pass有无数位的情况下），假设p1.pass=PASS_MAX, p2.pass=PASS_MIN，一定有PASS_MAX-PASS_MIN <= BigStride/2。否则在p1.pass上一次被执行时，一定有p2.pass <= PASS_MIN < PASS_MAX - BigStride/2 <= PASS_MAX - p1.stride = p1.pass，即此时p2.pass < p1.pass, p1不可能作为pass最小的进程被选择出来执行，和我们的假设矛盾。因而此时一定有PASS_MAX - PASS_MIN <= BigStride/2
  ```
  
- 已知以上结论，**考虑溢出的情况下**，可以为 Pass 设计特别的比较器，让 BinaryHeap\<Pass\> 的 pop 方法能返回真正最小的 Pass。补全下列代码中的 `partial_cmp` 函数，假设两个 Pass 永远不会相等。

```rust
use core::cmp::Ordering;

struct Pass(u64);

// 若两数之差的绝对值小于 BigStride/2，说明没有发生溢出，真实的大小关系应该和现在的大小关系相同 
// 若两数之差的绝对值大于 BigStride/2，说明发生了溢出，真实的大小关系应该和现在的大小关系相反 
impl PartialOrd for Pass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let half_big_stride= 127;
        if self.0 < other.0 {
            if other.0-self.0<=a {
                return Some(Ordering::Less);
            } 
            else {
                return Some(Ordering::Greater);
            }
        } 
        else {
            if self.0-other.0<=a {
                return Some(Ordering::Greater);
            } 
            else {
                return Some(Ordering::Less);
            }
        }
    }
}

impl PartialEq for Pass {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

*TIPS: 使用 8 bits 存储 pass, BigStride = 255, 则:* `(125 < 255) == false`*,* `(129 < 255) == true`*.*

--------------

### 感想和建议

虽然每次实验实现向前兼容的模块都会因为其他模块的修改有了一点点变化，但代码主要部分变化不大。由于实现的功能增多，每次为了前向兼容需要拷贝的代码过多，希望能够提供一种可以降低兼容复杂度的方法，如之前的仓库版本中不同分支之间`git merge`的操作。

