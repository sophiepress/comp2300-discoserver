/* We skip the normal startup script and start directly at main. This means .data and the sp are not initialised */
ENTRY(main)

/* Specifies the size and location of the memory areas */
MEMORY {
    FLASH   (rx)  : ORIGIN = 0x00000000,      LENGTH = 512K
    RAM     (rwx) : ORIGIN = 0x20000000,      LENGTH = 124K
    CODERAM (rwx) : ORIGIN = 0x00800000+124K, LENGTH = 4K
}

OUTPUT_FORMAT ("elf32-littlearm")

_stack_end = ORIGIN(RAM) + LENGTH(RAM); /* 0x2001_F000 */

_min_stack_size = 0x400;

SECTIONS {
    .rodata.vtable : { *(.rodata.vtable) } > FLASH

    .text : {
        . = ALIGN(4);
        _text_start = .;
        *(.text)
        _text_end = .;
    } > FLASH

    .data : {
        . = ALIGN(4);
        _data_start_ram = .;
        KEEP(*(.data))
        . = ALIGN(4);
        _data_end_ram = .;

    } > RAM AT> FLASH

    _data_start_flash = LOADADDR(.data);
    _data_end_flash = _data_start_flash + (_data_end_ram - _data_start_ram);
}
