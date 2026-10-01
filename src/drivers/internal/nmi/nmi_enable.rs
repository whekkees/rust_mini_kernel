use crate::arch::io::inb;
use crate::arch::io::outb;

pub unsafe fn nmi_enable() { 
    outb(0x70, inb(0x70) & 0x7F);
    inb(0x71);
}