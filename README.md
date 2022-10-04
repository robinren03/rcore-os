# rCore-Tutorial-Code-2022A

This is the code repository for the 2022 Autumn rCore Tutorial of OS course.

这是 2022 年秋季学期操作系统课程实验 rCore-Tutorial 的代码仓库。

**如有任何问题，请及时联系老师和助教。**

## 说明

仓库结构：

```
.
├── .env                    # * 用于填写实验信息，如已完成的最新实验
├── bootloader              # rustsbi 二进制文件
├── conf                    # ci 配置文件和 dockerfile  (此目录不用 git 跟踪)
├── ci-user                 # ci 使用的用户态程序和脚本   (此目录不用 git 跟踪)
├── easy-fs                 # easy-fs 文件系统
├── easy-fs-fuse
├── LICENSE
├── Makefile                # 顶层 Makefile
├── os1                     # * 第一章
├── os2                     # * 第二章
├── os3                     # * 第三章 lab1
├── os4                     # * 第四章 lab2
├── os5                     # * 第五章 lab3
├── os6                     # * 第六章
├── os7                     # * 第七章 lab4
├── os8                     # * 第八章 lab5
├── README.md               # 本文件
└── user -> ci-user/user    # 软链接到 ci 中的用户态程序
```

请根据实验进度，在相应的目录下编写代码，并在 `.env` 中填写已完成的最新实验信息（默认为 `FINISHED_LAB=lab1`）。

## 本地环境准备

> 本节假设你在一个 unix-like 系统上进行开发，如 Linux、macOS 等。

你可以执行以下命令来配置测试用的用户态程序：

```bash
# clone 测例仓库
make setup-user
```

**请不要随便修改 `ci-user` 目录下的文件和 `Makefile`，否则可能导致测试失败或与 CI 结果不同。**

当完成一次实验，如 `lab1`，首先修改 `.env` 中的 `FINISHED_LAB` 一项，然后在项目的根目录下执行如下指令可模拟 CI 中的测试

```bash
make ci-test
```

实验中如需测试，可在对应的 `os<X>` 目录中 `make run`。

### docker

如果你不想在本地安装 rust 等工具链，可以使用 docker 来进行开发。参考命令如下：

```bash
# clone conf 仓库
# 可直接执行下一跳命令
make setup-conf

# 构建 docker 镜像并进入容器，同时挂载当前目录
# 此命令已将上一条作为依赖，所以每次运行都会检查是否已存在 conf 目录
make docker
```
