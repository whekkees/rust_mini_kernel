use crate::arch::io::{inb, inw, outb, outw};
use crate::drivers::hardware::storage::ata::ata_error::AtaError;

const DATA_PORT: u16 = 0x1F0; // даныне 
const SECTOR_COUNT: u16 = 0x1F2; // работа с секторами
const LBA_LOW: u16 = 0x1F3;
const LBA_MID: u16 = 0x1F4;
const LBA_HIGH: u16 = 0x1F5;
const DRIVE_PORT: u16 = 0x1F6; // выбирает диск 
const STATUS_PORT: u16 = 0x1F7;

const ERR: u8 = 0x01; // байт отвечающий за ошибку 
const DRQ: u8 = 0x08; // байт который сообщает готовонсть передавать данные 
const DF: u8 = 0x20; // байт который сообщает о сербезной апаратной ошибке 
const BSY: u8 = 0x80; // байт который сообщает о том что диск не занят

const ATA_TIMEOUT: u32 = 1_000_000;

pub unsafe fn status() -> u8 {
    inb(STATUS_PORT)
}

pub unsafe fn wait_empty() -> Result<(), AtaError> { 
    let mut timeout = 0;

    loop {
        let current_status = status();

        if current_status & ERR != 0 {
            return Err(AtaError::Error);
        }

        if current_status & DF != 0 {
            return Err(AtaError::DeviceFault);
        }

        if current_status & BSY == 0 { 
            return Ok(());
        }

        timeout += 1; 

        if timeout >= ATA_TIMEOUT {
            return Err(AtaError::Timeout);
        }
    }
}

pub unsafe fn wait_drq() -> Result<(), AtaError> { 
    let mut timeout = 0;

    loop {
        let current_status = status();

        if current_status & ERR != 0 {
            return Err(AtaError::Error);
        }

        if current_status & DF != 0 {
            return Err(AtaError::DeviceFault);
        }
 
        if current_status & BSY == 0 && current_status & DRQ != 0 { 
            return Ok(());
        }

        timeout += 1;  

        if timeout >= ATA_TIMEOUT {
            return Err(AtaError::Timeout);
        }
    }
}

pub unsafe fn read_sector(sector: u32, buffer: &mut [u8; 512]) -> Result<(), AtaError> { 
    wait_empty()?; 

    outb(DRIVE_PORT, 0xE0 | ((sector >> 24) & 0x0F) as u8); 
    outb(SECTOR_COUNT, 1);
    outb(LBA_LOW,  ((sector >> 0)  & 0xFF) as u8);
    outb(LBA_MID,  ((sector >> 8)  & 0xFF) as u8);
    outb(LBA_HIGH, ((sector >> 16) & 0xFF) as u8);

    outb(STATUS_PORT, 0x20);

    wait_drq()?;

    for data_port in 0..256 {
        let bytes: u16 = inw(DATA_PORT);

        buffer[data_port * 2] = (bytes & 0xFF) as u8;
        buffer[data_port * 2 + 1] = (bytes >> 8) as u8;
    }

    Ok(())
}

pub unsafe fn write_sector(sector: u32, buffer: &[u8; 512]) -> Result<(), AtaError> {
    wait_empty()?;

    outb(DRIVE_PORT, 0xE0 | ((sector >> 24) & 0x0F) as u8);
    outb(SECTOR_COUNT, 1);
    outb(LBA_LOW, ((sector >> 0) & 0xFF) as u8);
    outb(LBA_HIGH, ((sector >> 16) & 0xFF) as u8);
    outb(LBA_MID, ((sector >> 8) & 0xFF) as u8);

    outb(STATUS_PORT, 0x30);

    wait_drq()?;

    for data_port in 0..256 {
        let low = buffer[data_port * 2];
        let high = buffer[data_port * 2 + 1];

        let bytes = ((high as u16) << 8) | (low as u16);

        outw(DATA_PORT, bytes);
    }

    Ok(())
}