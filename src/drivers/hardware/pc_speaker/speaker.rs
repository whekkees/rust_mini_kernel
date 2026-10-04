// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::arch::io::inb;
use crate::arch::io::outb;
use crate::drivers::internal::pit::pit;

const TMP_PORT: u16 = 0x61; // место для хранения временной информации

pub unsafe fn play_sound(frequency: u32) {
    pit::set_channel_2(frequency);

    let tmp = inb(TMP_PORT); 

    if tmp & 3 != 3 { 
        outb(TMP_PORT, tmp | 3);
    }
}
