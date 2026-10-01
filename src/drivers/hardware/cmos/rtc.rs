// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::drivers::hardware::cmos::cmos::read_register;
use crate::drivers::hardware::vga::terminal;


const SECOND_BYTE : u8 =  0x00;
const MINUTE_BYTE : u8 = 0x02;
const HOUR_BYTE : u8 = 0x04;
const YEAR_BYTE : u8 = 0x09;
const MONTH_BYTE : u8 = 0x08;

fn bcd_to_binary(bcd: u8) -> u8 {
    ((bcd & 0xF0) >> 4) * 10 + (bcd & 0x0F)
}

pub unsafe fn get_second() -> u8 { 
   bcd_to_binary(read_register(SECOND_BYTE))
}

pub unsafe fn get_minute() -> u8 { 
    bcd_to_binary(read_register(MINUTE_BYTE))
}

pub unsafe fn get_hour() -> u8 {
    bcd_to_binary(read_register(HOUR_BYTE))
}

pub unsafe fn get_year() -> u8 { 
    bcd_to_binary(read_register(YEAR_BYTE))
}

pub unsafe fn get_month() -> u8 { 
    bcd_to_binary(read_register(MONTH_BYTE))
}

pub unsafe fn put_number_2(number: u8) {
    terminal::put_char(b'0' + number / 10);
    terminal::put_char(b'0' + number % 10);
}

pub unsafe fn print_time() { 
    let hour = get_hour();
    let minute = get_minute();
    let second = get_second();
    let year = get_year();
    let month = get_month();

    put_number_2(hour);
    terminal::put_char(b':');
    put_number_2(minute);
    terminal::put_char(b':');
    put_number_2(second);

}

