// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use crate::arch::io::outb;

pub const PIT_FREQUENCY: u32 = 1_193_180;

pub unsafe fn set_channel_2(frequency: u32) {
    let divisor = PIT_FREQUENCY / frequency;

    outb(0x43, 0xB6);

    outb(0x42, divisor as u8);
    outb(0x42, (divisor >> 8) as u8);
}