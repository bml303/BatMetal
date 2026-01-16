ENTRY(_start)

SECTIONS
{
    . = 0x200000; 

    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }

    .bss : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    }

    . = ALIGN(16);
    __end = .;
}