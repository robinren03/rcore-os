
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 40
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_9_start
    .quad app_10_start
    .quad app_11_start
    .quad app_12_start
    .quad app_13_start
    .quad app_14_start
    .quad app_15_start
    .quad app_16_start
    .quad app_17_start
    .quad app_18_start
    .quad app_19_start
    .quad app_20_start
    .quad app_21_start
    .quad app_22_start
    .quad app_23_start
    .quad app_24_start
    .quad app_25_start
    .quad app_26_start
    .quad app_27_start
    .quad app_28_start
    .quad app_29_start
    .quad app_30_start
    .quad app_31_start
    .quad app_32_start
    .quad app_33_start
    .quad app_34_start
    .quad app_35_start
    .quad app_36_start
    .quad app_37_start
    .quad app_38_start
    .quad app_39_start
    .quad app_39_end

    .global _app_names
_app_names:
    .string "ch2b_bad_address"
    .string "ch2b_bad_instructions"
    .string "ch2b_bad_register"
    .string "ch2b_hello_world"
    .string "ch2b_power_3"
    .string "ch2b_power_5"
    .string "ch2b_power_7"
    .string "ch3_taskinfo"
    .string "ch3b_sleep"
    .string "ch3b_sleep1"
    .string "ch3b_yield0"
    .string "ch3b_yield1"
    .string "ch3b_yield2"
    .string "ch4_mmap0"
    .string "ch4_mmap1"
    .string "ch4_mmap2"
    .string "ch4_mmap3"
    .string "ch4_unmap"
    .string "ch4_unmap2"
    .string "ch5_exit0"
    .string "ch5_exit1"
    .string "ch5_getpid"
    .string "ch5_setprio"
    .string "ch5_spawn0"
    .string "ch5_spawn1"
    .string "ch5_stride"
    .string "ch5_stride0"
    .string "ch5_stride1"
    .string "ch5_stride2"
    .string "ch5_stride3"
    .string "ch5_stride4"
    .string "ch5_stride5"
    .string "ch5_usertest"
    .string "ch5b_exit"
    .string "ch5b_forktest"
    .string "ch5b_forktest2"
    .string "ch5b_forktest_simple"
    .string "ch5b_forktree"
    .string "ch5b_initproc"
    .string "ch5b_user_shell"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 3
app_0_start:
    .incbin "../user/build/elf/ch2b_bad_address.elf"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 3
app_1_start:
    .incbin "../user/build/elf/ch2b_bad_instructions.elf"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 3
app_2_start:
    .incbin "../user/build/elf/ch2b_bad_register.elf"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .align 3
app_3_start:
    .incbin "../user/build/elf/ch2b_hello_world.elf"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
    .align 3
app_4_start:
    .incbin "../user/build/elf/ch2b_power_3.elf"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
    .align 3
app_5_start:
    .incbin "../user/build/elf/ch2b_power_5.elf"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
    .align 3
app_6_start:
    .incbin "../user/build/elf/ch2b_power_7.elf"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
    .align 3
app_7_start:
    .incbin "../user/build/elf/ch3_taskinfo.elf"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
    .align 3
app_8_start:
    .incbin "../user/build/elf/ch3b_sleep.elf"
app_8_end:

    .section .data
    .global app_9_start
    .global app_9_end
    .align 3
app_9_start:
    .incbin "../user/build/elf/ch3b_sleep1.elf"
app_9_end:

    .section .data
    .global app_10_start
    .global app_10_end
    .align 3
app_10_start:
    .incbin "../user/build/elf/ch3b_yield0.elf"
app_10_end:

    .section .data
    .global app_11_start
    .global app_11_end
    .align 3
app_11_start:
    .incbin "../user/build/elf/ch3b_yield1.elf"
app_11_end:

    .section .data
    .global app_12_start
    .global app_12_end
    .align 3
app_12_start:
    .incbin "../user/build/elf/ch3b_yield2.elf"
app_12_end:

    .section .data
    .global app_13_start
    .global app_13_end
    .align 3
app_13_start:
    .incbin "../user/build/elf/ch4_mmap0.elf"
app_13_end:

    .section .data
    .global app_14_start
    .global app_14_end
    .align 3
app_14_start:
    .incbin "../user/build/elf/ch4_mmap1.elf"
app_14_end:

    .section .data
    .global app_15_start
    .global app_15_end
    .align 3
app_15_start:
    .incbin "../user/build/elf/ch4_mmap2.elf"
app_15_end:

    .section .data
    .global app_16_start
    .global app_16_end
    .align 3
app_16_start:
    .incbin "../user/build/elf/ch4_mmap3.elf"
app_16_end:

    .section .data
    .global app_17_start
    .global app_17_end
    .align 3
app_17_start:
    .incbin "../user/build/elf/ch4_unmap.elf"
app_17_end:

    .section .data
    .global app_18_start
    .global app_18_end
    .align 3
app_18_start:
    .incbin "../user/build/elf/ch4_unmap2.elf"
app_18_end:

    .section .data
    .global app_19_start
    .global app_19_end
    .align 3
app_19_start:
    .incbin "../user/build/elf/ch5_exit0.elf"
app_19_end:

    .section .data
    .global app_20_start
    .global app_20_end
    .align 3
app_20_start:
    .incbin "../user/build/elf/ch5_exit1.elf"
app_20_end:

    .section .data
    .global app_21_start
    .global app_21_end
    .align 3
app_21_start:
    .incbin "../user/build/elf/ch5_getpid.elf"
app_21_end:

    .section .data
    .global app_22_start
    .global app_22_end
    .align 3
app_22_start:
    .incbin "../user/build/elf/ch5_setprio.elf"
app_22_end:

    .section .data
    .global app_23_start
    .global app_23_end
    .align 3
app_23_start:
    .incbin "../user/build/elf/ch5_spawn0.elf"
app_23_end:

    .section .data
    .global app_24_start
    .global app_24_end
    .align 3
app_24_start:
    .incbin "../user/build/elf/ch5_spawn1.elf"
app_24_end:

    .section .data
    .global app_25_start
    .global app_25_end
    .align 3
app_25_start:
    .incbin "../user/build/elf/ch5_stride.elf"
app_25_end:

    .section .data
    .global app_26_start
    .global app_26_end
    .align 3
app_26_start:
    .incbin "../user/build/elf/ch5_stride0.elf"
app_26_end:

    .section .data
    .global app_27_start
    .global app_27_end
    .align 3
app_27_start:
    .incbin "../user/build/elf/ch5_stride1.elf"
app_27_end:

    .section .data
    .global app_28_start
    .global app_28_end
    .align 3
app_28_start:
    .incbin "../user/build/elf/ch5_stride2.elf"
app_28_end:

    .section .data
    .global app_29_start
    .global app_29_end
    .align 3
app_29_start:
    .incbin "../user/build/elf/ch5_stride3.elf"
app_29_end:

    .section .data
    .global app_30_start
    .global app_30_end
    .align 3
app_30_start:
    .incbin "../user/build/elf/ch5_stride4.elf"
app_30_end:

    .section .data
    .global app_31_start
    .global app_31_end
    .align 3
app_31_start:
    .incbin "../user/build/elf/ch5_stride5.elf"
app_31_end:

    .section .data
    .global app_32_start
    .global app_32_end
    .align 3
app_32_start:
    .incbin "../user/build/elf/ch5_usertest.elf"
app_32_end:

    .section .data
    .global app_33_start
    .global app_33_end
    .align 3
app_33_start:
    .incbin "../user/build/elf/ch5b_exit.elf"
app_33_end:

    .section .data
    .global app_34_start
    .global app_34_end
    .align 3
app_34_start:
    .incbin "../user/build/elf/ch5b_forktest.elf"
app_34_end:

    .section .data
    .global app_35_start
    .global app_35_end
    .align 3
app_35_start:
    .incbin "../user/build/elf/ch5b_forktest2.elf"
app_35_end:

    .section .data
    .global app_36_start
    .global app_36_end
    .align 3
app_36_start:
    .incbin "../user/build/elf/ch5b_forktest_simple.elf"
app_36_end:

    .section .data
    .global app_37_start
    .global app_37_end
    .align 3
app_37_start:
    .incbin "../user/build/elf/ch5b_forktree.elf"
app_37_end:

    .section .data
    .global app_38_start
    .global app_38_end
    .align 3
app_38_start:
    .incbin "../user/build/elf/ch5b_initproc.elf"
app_38_end:

    .section .data
    .global app_39_start
    .global app_39_end
    .align 3
app_39_start:
    .incbin "../user/build/elf/ch5b_user_shell.elf"
app_39_end:
