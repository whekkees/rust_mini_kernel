// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::drivers::hardware::cmos::cmos::write_register;
use crate::drivers::hardware::cmos::cmos::read_register;

pub unsafe fn write(register: u8, val: u8) {  // записывать значенеи в cmos регистр 
    write_register(register, val);
}

pub unsafe fn read(register: u8) -> u8 { // читать значение из cmos регистра 
    read_register(register)
}