
pub struct Elf32Header {
    pub class : u8,
    pub endianness : u8,
    pub machine: u16,
    pub entry : u32,
    pub program_header_offset : u32,
    pub program_header_size : u16,
    pub program_header_count : u16
}

pub fn elf_check(data: &[u8]) -> Result<Elf32Header, &'static str> {
    if data.len() < 4 {
        return Err("Data too small");
    }

    if data[0] != 0x7F|| data[1] != b'E'|| data[2] != b'L'|| data[3] != b'F' {
        return Err("Invalid magic");
    }

    if data.len() < 5 {
        return Err("Data too small");
    }

    if data[4] != 1 {
        return Err("Invalid class");
    }

    let class = data[4];

    if data.len() < 6 {
        return Err("Data too small");
    }

    let endianness = data[5];

    if data[5] != 1 {
        return Err("Invalid endianness");
    }

    if data.len() < 20 {
        return Err("Data too small");
    }

    let machine = u16::from_le_bytes([data[18], data[19]]);

    if machine != 3 {
        return Err("Invalid machine")
    }

    if data.len() < 28 {
        return Err("Data too small")
    }

    let entry = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);

    if data.len() < 32 {
        return Err("Data too small")
    }

    let program_header_offset = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);


    if data.len() < 44  {
        return Err("Data too small")
    }

    let program_header_size = u16::from_le_bytes([data[42], data[43]]);


    if data.len() < 46 {
        return Err("Data too small")
    }

    let program_header_count = u16::from_le_bytes([data[44], data[45]]);


    let header = Elf32Header { class, endianness, machine, entry, program_header_offset, program_header_size, program_header_count };

    Ok(header)
}