use crate::arch::io::{inl, outl};

const CONFIG_ADRESS : u16 = 0xCF8;
const CONFIG_DATA : u16 = 0xCFC;

pub unsafe fn read_pci (bus : u8, device : u8, function : u8, offset : u8) -> u32 {

    let busl = bus as u32;
    let devicel = device as u32;
    let functionl = function as u32;
    let offsetl = offset as u32;

    let adress = 0x80000000 | (busl << 16) | (devicel << 11) | (functionl << 8) | (offsetl & 0xFC);
    outl(CONFIG_ADRESS, adress);
    inl(CONFIG_DATA)

}

pub unsafe fn write_pci () {

}