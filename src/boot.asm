BITS 32

global start
extern kmain


MULTIBOOT_MAGIC    equ 0x1BADB002
MULTIBOOT_FLAGS    equ 0x00000003
MULTIBOOT_CHECKSUM equ -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

section .multiboot
align 4

    dd MULTIBOOT_MAGIC
    dd MULTIBOOT_FLAGS
    dd MULTIBOOT_CHECKSUM


section .text
align 16

start:
    cli

    mov esp, stack_top

    push ebx
    push eax

    call kmain


.hang:
    cli
    hlt
    jmp .hang


section .bss
align 16

stack_bottom:
    resb 65536

stack_top:


section .note.GNU-stack