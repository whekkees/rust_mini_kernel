// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use crate::drivers::hardware::ps2::ps2::{read_data,wait_output_full};
use super::ascii;
use super::key::Key;
use super::scancode::{self, KeyEvent};
use crate::drivers::hardware::ps2_keyboard::scancode::decode;

static mut SHIFT: bool = false;
static mut CAPS_LOCK: bool = false;

pub unsafe fn read_event() -> KeyEvent { // ждет данные от ps 2 и потом декодирует в key event 
    loop {
        wait_output_full();

        let scancode = read_data();

        if let Some(event) = decode(scancode) {
            update_state(event);
            return event;
        }
    }
}

unsafe fn update_state(event: KeyEvent) { // состояние клавиш при нажатии на шифт или капслок 
    match event.key {
        Key::LeftShift | Key::RightShift => {
            SHIFT = event.pressed;
        }

        Key::CapsLock => {
            if event.pressed {
                CAPS_LOCK = !CAPS_LOCK;
            }
        }

        _ => {}
    }
}

pub unsafe fn shift() -> bool { // проверяет ли нажат шифт 
    SHIFT
}

pub unsafe fn caps_lock() -> bool { // проверяет ли нажат капс лок 
    CAPS_LOCK
}

pub unsafe fn to_ascii(key: Key) -> Option<u8> { // передает клавишу в to_ascii из ascii.rs
    ascii::to_ascii(key,SHIFT,CAPS_LOCK,)
}