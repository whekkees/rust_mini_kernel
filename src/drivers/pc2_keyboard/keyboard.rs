// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

use super::ps2;
use super::scancode::{self, Key, KeyEvent};

static mut SHIFT: bool = false;
static mut CAPS_LOCK: bool = false;

pub unsafe fn read_event() -> KeyEvent {
    loop {
        ps2::wait_output_full();

        let scancode = ps2::read_data();

        if scancode == 0xE0 {
            ps2::wait_output_full();

            let _extended = ps2::read_data();
            continue;
        }

        if let Some(event) = scancode::decode(scancode) {
            update_state(event);

            return event;
        }
    }
}

unsafe fn update_state(event: KeyEvent) {
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

pub unsafe fn shift() -> bool {
    SHIFT
}

pub unsafe fn caps_lock() -> bool {
    CAPS_LOCK
}