.code32

.global start
.extern kmain

.section .multiboot
.align 4

.set MULTIBOOT_MAGIC,    0x1BADB002
.set MULTIBOOT_FLAGS,    0x00000003
.set MULTIBOOT_CHECKSUM, -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

.long MULTIBOOT_MAGIC
.long MULTIBOOT_FLAGS
.long MULTIBOOT_CHECKSUM


.section .text

start:
    cli

    mov esp, offset stack_top

    push ebx
    push eax

    call kmain

.hang:
    cli
    hlt
    jmp .hang


.section .bss
.align 16

stack_bottom:
    .skip 65536

stack_top:


.section .note.GNU-stack