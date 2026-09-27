ENTRY(start)

SECTIONS {
    . = 1M; /* Загружаем ядро по адресу 1 Мегабайт */

    .text : {
        KEEP(*(.multiboot)) /* Фиксируем Multiboot-заголовок в самом начале */
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }
    .bss    : { *(.bss .bss.*) }
}