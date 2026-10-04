
use crate::drivers::hardware::storage::ata::ata::{read_sector,write_sector};
use crate::drivers::hardware::storage::ata::ata_error::AtaError;

pub trait Storage { 
    unsafe fn write_sector(&self, sector: u32, buffer: &[u8;512]) -> Result<(), AtaError>; 
    unsafe fn read_sector(&self, sector : u32, buffer: &mut[u8;512]) -> Result<(), AtaError>;
}

pub struct AtaDisk;

impl Storage for AtaDisk {

    unsafe fn write_sector(&self, sector: u32, buffer: &[u8;512]) -> Result<(), AtaError> {
        write_sector(sector, buffer)
    }

    unsafe fn read_sector(&self, sector: u32, buffer: &mut[u8;512]) -> Result<(), AtaError> {
        read_sector(sector, buffer)
    }
}
