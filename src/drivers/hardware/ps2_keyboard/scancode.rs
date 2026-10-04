// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)


use crate::drivers::hardware::ps2_keyboard::key::Key;


const RELEASED: u8 = 0x80;
#[derive(Debug, Clone, Copy)]
pub struct KeyEvent { // евент для указывания клавиши и проверки ли нажата кнопка
    pub key: Key,
    pub pressed: bool,
}

pub fn decode(scancode: u8) -> Option<KeyEvent> { // декодировка скан кодов 
    let pressed = scancode & RELEASED == 0; // 0x80 означает что клавиша отпущена 
    let code = scancode & 0x7F;

    let key = match code {
        0x01 => Key::Escape,

        0x02 => Key::Num1,
        0x03 => Key::Num2,
        0x04 => Key::Num3,
        0x05 => Key::Num4,
        0x06 => Key::Num5,
        0x07 => Key::Num6,
        0x08 => Key::Num7,
        0x09 => Key::Num8,
        0x0A => Key::Num9,
        0x0B => Key::Num0,

        0x0C => Key::Minus,
        0x0D => Key::Equals,
        0x0E => Key::Backspace,
        0x0F => Key::Tab,

        0x10 => Key::Q,
        0x11 => Key::W,
        0x12 => Key::E,
        0x13 => Key::R,
        0x14 => Key::T,
        0x15 => Key::Y,
        0x16 => Key::U,
        0x17 => Key::I,
        0x18 => Key::O,
        0x19 => Key::P,

        0x1C => Key::Enter,

        0x1E => Key::A,
        0x1F => Key::S,
        0x20 => Key::D,
        0x21 => Key::F,
        0x22 => Key::G,
        0x23 => Key::H,
        0x24 => Key::J,
        0x25 => Key::K,
        0x26 => Key::L,


        0x2A => Key::LeftShift,

        0x2C => Key::Z,
        0x2D => Key::X,
        0x2E => Key::C,
        0x2F => Key::V,
        0x30 => Key::B,
        0x31 => Key::N,
        0x32 => Key::M,

        0x36 => Key::RightShift,
        0x39 => Key::Space,

        0x45 => Key::CapsLock,

        _ => return None,
    };

    Some(KeyEvent {key,pressed})
}