// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use crate::drivers::io::{inb::inb, outb::outb};

const DATA_PORT : u16 = 0x60;
const STATUS_PORT : u16 = 0x64;


pub unsafe fn read_data() -> u8 {
    inb(DATA_PORT)
}

pub unsafe fn status() -> u8 {
    inb(STATUS_PORT)
}

pub unsafe  fn write_data (data : u8) {
    wait_input_empty();

    outb(DATA_PORT, data);
}

pub unsafe  fn wait_input_empty() {
    while status() & 0x02 != 0 {}
}

pub unsafe fn wait_output_full() {
    while status() & 0x01 == 0 {}

}