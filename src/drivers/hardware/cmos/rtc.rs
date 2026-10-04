// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::drivers::hardware::cmos::cmos::read_register;
use crate::drivers::hardware::vga::terminal;


const SECOND_BYTE : u8 =  0x00; // секунды 
const MINUTE_BYTE : u8 = 0x02; // минуты
const HOUR_BYTE : u8 = 0x04; // часы 
const YEAR_BYTE : u8 = 0x09; // года
const MONTH_BYTE : u8 = 0x08; // месяца 

pub unsafe fn get_second() -> u8 {  // получить секунды
   read_register(SECOND_BYTE)
}

pub unsafe fn get_minute() -> u8 { // получить минуты
    read_register(MINUTE_BYTE)
}

pub unsafe fn get_hour() -> u8 { // получить часы
    read_register(HOUR_BYTE)
}

pub unsafe fn get_year() -> u8 { // получить год
    read_register(YEAR_BYTE)
}

pub unsafe fn get_month() -> u8 {  // получить месяц
    read_register(MONTH_BYTE)
}

/*

добавить расшифровку бсд и перевод в ascii позже

 */