#![no_std]
#![no_main]

use crate::kernel::kmain::start;

pub mod cpp;
pub mod arch;
pub mod drivers;
pub mod kernel;
pub mod panic;
pub mod lib;


// логика запуска ядра 

#[unsafe(no_mangle)]
pub extern "C" fn kmain(magic: u32, multiboot_info_ptr: u32) -> ! {
    start(magic, multiboot_info_ptr);
}