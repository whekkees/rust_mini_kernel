use super::key::Key;

pub fn to_ascii(key: Key, shift: bool, caps_lock: bool) -> Option<u8> {
    match key {
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
        Key::Backspace => Some(8),
        Key::Tab => Some(b'\t'),

        Key::Minus => Some(if shift { b'_' } else { b'-' }),
        Key::Equals => Some(if shift { b'+' } else { b'=' }),

        _ => None,
    }
}


fn letter(lw: u8,  shift: bool, caps_lock: bool) -> u8 {
    let up = shift ^ caps_lock;

    if up {
        lw - b'a' + b'A'
    } else {
        lw
    }
}

/*
false ^ false = false
true  ^ false = true
false ^ true  = true
true  ^ true  = false
 */