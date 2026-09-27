// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

#![no_std]
#![no_main]

use core::arch::global_asm;

use crate::drivers::speaker::play_sound;

mod panic;
mod drivers;

global_asm!(include_str!("boot.asm"));

static HELLO: &[u8] = b"Hello World213213";

#[unsafe(no_mangle)]
pub extern "C" fn kmain(magic: u32, _multiboot_info_ptr: u32) -> ! {
    
    if magic == 0x2BADB002 {
    }

    unsafe {
    play_sound(1000);
    }

    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        for i in 0..2000 {
            *vga_buffer.add(i * 2) = b' ';
            *vga_buffer.add(i * 2 + 1) = 0x07;
        }
    }

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = 0x0b;
        }
    }

    loop {}
}