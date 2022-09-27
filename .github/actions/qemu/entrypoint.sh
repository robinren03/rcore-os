#!/bin/sh

# check qemu and rust
qemu-system-riscv64 --version
rustc --version
cargo --version

cd $GITHUB_WORKSPACE
make ci-test | tee qemu_run_output.txt
