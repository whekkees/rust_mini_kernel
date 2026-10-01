use super::vga;

pub unsafe fn put_char(byte: u8) {
    match byte {
        b'\n' => newline(),
        8 => backspace(),

        _ => {
            vga::write_byte(byte);

            let mut x = vga::position_x() + 1;
            let mut y = vga::position_y();

            if x >= vga::WIDTH {
                x = 0;
                y += 1;
            }

            if y >= vga::HEIGHT {
                y = 0;
            }

            vga::set_position(x, y);
        }
    }
}

pub unsafe fn print(text: &[u8]) {
    for byte in text {
        put_char(*byte);
    }
}

unsafe fn newline() {
    let mut y = vga::position_y() + 1; 

    if y >= vga::HEIGHT {
        y = 0;
    }

    // просто опускаем
    vga::set_position(0, y);
}

unsafe fn backspace() {
    let mut x = vga::position_x();

    if x == 0 {
        return;
    }

    x -= 1;

    vga::set_position(x, vga::position_y());
    vga::erase_byte();
}