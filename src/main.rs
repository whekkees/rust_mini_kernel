#![no_std]
#![no_main]

mod drivers;
mod panic;

const MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x2BADB002;

#[unsafe(no_mangle)]
pub extern "C" fn kmain(magic: u32, multiboot_info_ptr: u32) -> ! {
    if magic != MULTIBOOT_BOOTLOADER_MAGIC {
        halt();
    }

    let _ = multiboot_info_ptr;

    unsafe {
        drivers::vga::vga::clear();

        drivers::vga::vga::put_char(b'A');
        drivers::vga::vga::put_char(b'v');
        drivers::vga::vga::put_char(b'e');
        drivers::vga::vga::put_char(b'r');
        drivers::vga::vga::put_char(b'i');
        drivers::vga::vga::put_char(b'o');
        drivers::vga::vga::put_char(b'n');
        drivers::vga::vga::put_char(b'>');
        drivers::vga::vga::put_char(b' ');
    }

    loop {
        let event = unsafe {
            drivers::pc2_keyboard::keyboard::read_event()
        };

        if !event.pressed {
            continue;
        }

        match event.key {
            drivers::pc2_keyboard::scancode::Key::Backspace => {
                unsafe {
                    drivers::vga::vga::put_char(b'\x08');
                }
            }

            _ => {
                let shift = unsafe {
                    drivers::pc2_keyboard::keyboard::shift()
                };

                let caps_lock = unsafe {
                    drivers::pc2_keyboard::keyboard::caps_lock()
                };

                if let Some(character) = event.key.ascii(shift, caps_lock) {
                    unsafe {
                        drivers::vga::vga::put_char(character);
                    }
                }
            }
        }
    }
}

fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("cli", "hlt");
        }
    }
}