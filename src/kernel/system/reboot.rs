use crate::{arch::io::inb, drivers::hardware::ps2::ps2::wait_input_empty};
use crate::arch::io::outb;

const CONTROLLER_STATUS : u16 = 0x64;

pub unsafe fn reboot_computer() { 
    wait_input_empty();
    outb(CONTROLLER_STATUS, 0xFE);
}