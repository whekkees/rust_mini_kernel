// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(__panic_information : &PanicInfo) -> ! {
    
    loop {

    }
}