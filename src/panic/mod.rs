use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(__panic_information : &PanicInfo) -> ! {
    
    loop {

    }
}