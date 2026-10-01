use core::panic::PanicInfo;
use crate::{arch::cpu, drivers::hardware::vga::terminal};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        terminal::print(b"AVERION KERNEL PANIC");
    }

    cpu::halt()
}
