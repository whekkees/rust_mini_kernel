// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use crate::arch::io::inb;
use crate::arch::io::outb;

const DATA_PORT: u16 = 0x60; // порт данных 
const STATUS_PORT: u16 = 0x64; // порт состояния либо команд

pub unsafe fn read_data() -> u8 { // читаем данные из порт 0x60
    inb(DATA_PORT)
}

pub unsafe fn status() -> u8 { // получаем статус порта 0x64
    inb(STATUS_PORT)
}

pub unsafe fn write_data(data: u8) { // записываем данные в дата порт
    wait_input_empty();
    outb(DATA_PORT, data);
}

pub unsafe fn send_command(command: u8) { // отправляем команду контроллеру ps2 
    wait_input_empty();
    outb(STATUS_PORT, command);
}

pub unsafe fn wait_input_empty() { // ждем пока освободится место 
    while status() & 0x02 != 0 {}
}

pub unsafe fn wait_output_full() { // ждем пока появятся данные для чтения
    while status() & 0x01 == 0 {}
}

pub unsafe fn output_full() -> bool { // проверяем ли есть данные для чтения 
    status() & 0x01 != 0
}
