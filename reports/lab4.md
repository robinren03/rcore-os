# Lab4实验报告

任彦羽 2019011215

### 实现功能

1. 在进行新旧文件名比较后，循环在各个`DirEntry`中找到旧文件路径的`inode_number`，然后使用找到的`inode_number`和新文件路径创建新的`DirEntry`并写到root_inode中，实现`sys_linkat`。
2. 循环在各个`DirEntry`中找到要删除的文件路径的`inode_number`，如果这是最后一个有着该`inode_number`的entry则清除文件内容。再`root_inode`的相应位置写入一个空的`DirEntry`覆盖原有信息，实现`sys_unlinkat`。
3. 进行`translated_refmut`后通过`id`和`offset`计算得到`ino`，并得到其他信息，实现`sys_stat`。

-----------

### 问答题

1. 在我们的 easy-fs 中，root inode 起着什么作用？如果 root inode 中的内容损坏了，会发生什么？

答：root inode是根目录的 inode，可以查找根目录内容，和其它文件的索引。 如果发生损坏，操作系统（用户）将无法找到其它文件，出现文件丢失问题。

2. 举出使用 pipe 的一个实际应用的例子。

答：```lscpu | grep "cache"```

查看CPU信息并输出其中关于cache的信息。`grep`进程需要通过管道获得`lscpu`进程所发送（输出）的信息。

3. 如果需要在多个进程间互相通信，则需要为每一对进程建立一 个管道，非常繁琐，请设计一个更易用的多进程通信机制。

答：可以采取一个集中的结构，fork 一个只用来集中/分发通信的进程。其它进程通过管道向它发送信息，并发送该信息的目的进程，然后再由fork出来的进程转发到真正的目的进程。
