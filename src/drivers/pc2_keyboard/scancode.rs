// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

#[derive(Debug, Clone, Copy)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    Space,
    Enter,
    Backspace,
    Tab,
    Escape,

    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Dot,
    Slash,

    LeftShift,
    RightShift,
    CapsLock,

    LeftCtrl,
    RightCtrl,

    LeftAlt,
    RightAlt,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub key: Key,
    pub pressed: bool,
}

pub fn decode(scancode: u8) -> Option<KeyEvent> {
    let pressed = scancode & 0x80 == 0;
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

        0x1A => Key::LeftBracket,
        0x1B => Key::RightBracket,
        0x1C => Key::Enter,

        0x1D => Key::LeftCtrl,

        0x1E => Key::A,
        0x1F => Key::S,
        0x20 => Key::D,
        0x21 => Key::F,
        0x22 => Key::G,
        0x23 => Key::H,
        0x24 => Key::J,
        0x25 => Key::K,
        0x26 => Key::L,

        0x27 => Key::Semicolon,
        0x28 => Key::Apostrophe,
        0x29 => Key::Grave,

        0x2A => Key::LeftShift,
        0x2B => Key::Backslash,

        0x2C => Key::Z,
        0x2D => Key::X,
        0x2E => Key::C,
        0x2F => Key::V,
        0x30 => Key::B,
        0x31 => Key::N,
        0x32 => Key::M,

        0x33 => Key::Comma,
        0x34 => Key::Dot,
        0x35 => Key::Slash,

        0x36 => Key::RightShift,

        0x38 => Key::LeftAlt,
        0x39 => Key::Space,

        0x3B => Key::F1,
        0x3C => Key::F2,
        0x3D => Key::F3,
        0x3E => Key::F4,
        0x3F => Key::F5,
        0x40 => Key::F6,
        0x41 => Key::F7,
        0x42 => Key::F8,
        0x43 => Key::F9,
        0x44 => Key::F10,

        0x45 => Key::CapsLock,

        0x57 => Key::F11,
        0x58 => Key::F12,

        _ => return None,
    };

    Some(KeyEvent {
        key,
        pressed,
    })
}

impl Key {
    pub fn ascii(self, shift: bool, caps_lock: bool) -> Option<u8> {
        match self {
            Key::A => Some(letter(b'a', shift, caps_lock)),
            Key::B => Some(letter(b'b', shift, caps_lock)),
            Key::C => Some(letter(b'c', shift, caps_lock)),
            Key::D => Some(letter(b'd', shift, caps_lock)),
            Key::E => Some(letter(b'e', shift, caps_lock)),
            Key::F => Some(letter(b'f', shift, caps_lock)),
            Key::G => Some(letter(b'g', shift, caps_lock)),
            Key::H => Some(letter(b'h', shift, caps_lock)),
            Key::I => Some(letter(b'i', shift, caps_lock)),
            Key::J => Some(letter(b'j', shift, caps_lock)),
            Key::K => Some(letter(b'k', shift, caps_lock)),
            Key::L => Some(letter(b'l', shift, caps_lock)),
            Key::M => Some(letter(b'm', shift, caps_lock)),
            Key::N => Some(letter(b'n', shift, caps_lock)),
            Key::O => Some(letter(b'o', shift, caps_lock)),
            Key::P => Some(letter(b'p', shift, caps_lock)),
            Key::Q => Some(letter(b'q', shift, caps_lock)),
            Key::R => Some(letter(b'r', shift, caps_lock)),
            Key::S => Some(letter(b's', shift, caps_lock)),
            Key::T => Some(letter(b't', shift, caps_lock)),
            Key::U => Some(letter(b'u', shift, caps_lock)),
            Key::V => Some(letter(b'v', shift, caps_lock)),
            Key::W => Some(letter(b'w', shift, caps_lock)),
            Key::X => Some(letter(b'x', shift, caps_lock)),
            Key::Y => Some(letter(b'y', shift, caps_lock)),
            Key::Z => Some(letter(b'z', shift, caps_lock)),

            Key::Num0 => Some(if shift { b')' } else { b'0' }),
            Key::Num1 => Some(if shift { b'!' } else { b'1' }),
            Key::Num2 => Some(if shift { b'@' } else { b'2' }),
            Key::Num3 => Some(if shift { b'#' } else { b'3' }),
            Key::Num4 => Some(if shift { b'$' } else { b'4' }),
            Key::Num5 => Some(if shift { b'%' } else { b'5' }),
            Key::Num6 => Some(if shift { b'^' } else { b'6' }),
            Key::Num7 => Some(if shift { b'&' } else { b'7' }),
            Key::Num8 => Some(if shift { b'*' } else { b'8' }),
            Key::Num9 => Some(if shift { b'(' } else { b'9' }),

            Key::Space => Some(b' '),
            Key::Enter => Some(b'\n'),
            Key::Tab => Some(b'\t'),

            Key::Minus => Some(if shift { b'_' } else { b'-' }),
            Key::Equals => Some(if shift { b'+' } else { b'=' }),

            Key::LeftBracket => Some(if shift { b'{' } else { b'[' }),
            Key::RightBracket => Some(if shift { b'}' } else { b']' }),
            Key::Backslash => Some(if shift { b'|' } else { b'\\' }),

            Key::Semicolon => Some(if shift { b':' } else { b';' }),
            Key::Apostrophe => Some(if shift { b'"' } else { b'\'' }),
            Key::Grave => Some(if shift { b'~' } else { b'`' }),

            Key::Comma => Some(if shift { b'<' } else { b',' }),
            Key::Dot => Some(if shift { b'>' } else { b'.' }),
            Key::Slash => Some(if shift { b'?' } else { b'/' }),

            _ => None,
        }
    }
}

fn letter(lowercase: u8, shift: bool, caps_lock: bool) -> u8 {
    let uppercase = shift ^ caps_lock;

    if uppercase {
        lowercase - b'a' + b'A'
    } else {
        lowercase
    }
}