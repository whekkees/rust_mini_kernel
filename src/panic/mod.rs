// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(__panic_information : &PanicInfo) -> ! {
    
    loop {

    }
}


/*
New-Item -ItemType Directory -Force target
nasm -f elf32 src\boot.asm -o target\boot.o
cargo run
 */