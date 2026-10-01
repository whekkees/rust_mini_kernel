use core::ptr;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;
const COLOR: u8 = 0x0F;

static mut POSITION_X: usize = 0;
static mut POSITION_Y: usize = 0;

pub unsafe fn position_x() -> usize {
    POSITION_X
}

pub unsafe fn position_y() -> usize {
    POSITION_Y
}

pub unsafe fn set_position(x: usize, y: usize) {
    POSITION_X = x;
    POSITION_Y = y;
}

pub unsafe fn write_byte(byte: u8) {
    let offset = (POSITION_Y * WIDTH + POSITION_X) * 2;

    ptr::write_volatile(VGA_BUFFER.add(offset), byte);
    ptr::write_volatile(VGA_BUFFER.add(offset + 1), COLOR);
}

pub unsafe fn erase_byte() {
    write_byte(b' ');
}