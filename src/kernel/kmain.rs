
use crate::arch::cpu; 
use crate::drivers::hardware::vga::terminal::put_char;
use crate::drivers::hardware::vga;
use crate::drivers::hardware::ps2_keyboard::keyboard::{read_event,to_ascii};
use crate::kernel::system::reboot::reboot_computer;

const MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x2BADB002;

pub fn start(magic: u32, multiboot_info_ptr: u32) -> ! {

    if magic != MULTIBOOT_BOOTLOADER_MAGIC {
        cpu::halt();
    }

    let _ = multiboot_info_ptr; // информация от GRUB

    unsafe {
        vga::terminal::print(b"Averion OS 0.1.0\n");
        vga::terminal::print(b"Copyright (C) 2026 whekkees\n\n");
        vga::terminal::print(b"Welcome to Averion!\n\n");
        vga::terminal::print(b"Press R to reboot.\n\n");
        vga::terminal::print(b"averion> \n");
        
    }

    loop {
        let event = unsafe {
            read_event()
        };

        if !event.pressed {
            continue;
        }

        if let Some(character) = unsafe {
            to_ascii(event.key)
        } {
        
        if character == b'r' || character == b'R' {
            unsafe {
                reboot_computer();
            }
        }

        unsafe {
            put_char(character);
        }
    }
}
}