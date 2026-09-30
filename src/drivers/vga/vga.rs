// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 whekkees (Daniil)

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;

const DEFAULT_COLOR: u8 = 0x0F;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;

pub unsafe fn clear() {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let offset = (y * WIDTH + x) * 2;

            VGA_BUFFER.add(offset).write_volatile(b' ');
            VGA_BUFFER.add(offset + 1).write_volatile(DEFAULT_COLOR);
        }
    }

    CURSOR_X = 0;
    CURSOR_Y = 0;
}

pub unsafe fn put_char(byte: u8) {
    match byte {
        b'\n' => {
            newline();
        }

        b'\t' => {
            for _ in 0..4 {
                put_char(b' ');
            }
        }

        b'\x08' => {
            backspace();
        }

        _ => {
            let offset = (CURSOR_Y * WIDTH + CURSOR_X) * 2;

            VGA_BUFFER.add(offset).write_volatile(byte);
            VGA_BUFFER.add(offset + 1).write_volatile(DEFAULT_COLOR);

            CURSOR_X += 1;

            if CURSOR_X >= WIDTH {
                newline();
            }
        }
    }
}

unsafe fn newline() {
    CURSOR_X = 0;
    CURSOR_Y += 1;

    if CURSOR_Y >= HEIGHT {
        scroll();
    }
}

unsafe fn backspace() {
    if CURSOR_X > 0 {
        CURSOR_X -= 1;

        let offset = (CURSOR_Y * WIDTH + CURSOR_X) * 2;

        VGA_BUFFER.add(offset).write_volatile(b' ');
        VGA_BUFFER.add(offset + 1).write_volatile(DEFAULT_COLOR);
    }
}

unsafe fn scroll() {
    for y in 1..HEIGHT {
        for x in 0..WIDTH {
            let src = (y * WIDTH + x) * 2;
            let dst = ((y - 1) * WIDTH + x) * 2;

            let character = VGA_BUFFER.add(src).read_volatile();
            let color = VGA_BUFFER.add(src + 1).read_volatile();

            VGA_BUFFER.add(dst).write_volatile(character);
            VGA_BUFFER.add(dst + 1).write_volatile(color);
        }
    }

    let last_line = (HEIGHT - 1) * WIDTH * 2;

    for x in 0..WIDTH {
        let offset = last_line + x * 2;

        VGA_BUFFER.add(offset).write_volatile(b' ');
        VGA_BUFFER.add(offset + 1).write_volatile(DEFAULT_COLOR);
    }

    CURSOR_Y = HEIGHT - 1;
}