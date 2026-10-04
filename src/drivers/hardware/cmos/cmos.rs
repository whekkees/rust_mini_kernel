// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::arch::io::inb;
use crate::arch::io::outb;

const CMOS_REGISTER_CHOOSE : u16 = 0x70;
const CMOS_REGISTER_DATA : u16 = 0x71;

pub unsafe fn read_register(register: u8) -> u8 {

    outb(CMOS_REGISTER_CHOOSE, register);
    inb(CMOS_REGISTER_DATA)
 }

 pub unsafe fn write_register(register: u8, val: u8) {
    
    outb(CMOS_REGISTER_CHOOSE, register);
    outb(CMOS_REGISTER_DATA, val);
 }

 /*
 0x70 - выбирает регистр 
 0x71 - записывает либо читает данные 
  */