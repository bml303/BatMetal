.section ".text.boot"

.global _start

_start:
    mrs     x0, mpidr_el1
    and     x0, x0, #0xFF
    cbnz    x0, .L_parking_loop

.L_setup_stack:
    ldr     x0, =0x400000 
    mov     sp, x0

    ldr     x1, =__bss_start
    ldr     x2, =__bss_end

.L_bss_loop:
    cmp     x1, x2
    bge     .L_jump_rust
    str     xzr, [x1], #8
    b       .L_bss_loop

.L_jump_rust:
    bl      kernel_main

.L_parking_loop:
    wfe
    b       .L_parking_loop