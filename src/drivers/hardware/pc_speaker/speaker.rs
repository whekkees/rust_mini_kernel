// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::arch::io::inb;
use crate::arch::io::outb;
use crate::drivers::internal::pit::pit;

pub unsafe fn play_sound(frequency: u32) {
    pit::set_channel_2(frequency);

    let tmp = inb(0x61); 

    if tmp & 0b00000011 != 0b00000011 {
        outb(0x61, tmp | 0b00000011);
    }
}
