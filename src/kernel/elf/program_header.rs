
pub struct Elf32ProgramHeader {
    pub segment_type: u32,
    pub offset: u32,
    pub virtual_address: u32,
    pub physical_address: u32,
    pub file_size: u32,
    pub memory_size: u32,
    pub flags: u32,
    pub align: u32,
}

pub fn parse_program_header(data: &[u8], offset_header: usize) -> Result<Elf32ProgramHeader, &'static str> {

    if offset_header + 32 > data.len() {
        return Err("Data too small")
    }


    let segment_type = u32::from_le_bytes([data[offset_header], data[offset_header + 1], data[offset_header + 2], data[offset_header + 3]]);
    let offset = u32::from_le_bytes([data[offset_header + 4], data[offset_header + 5], data[offset_header + 6], data[offset_header + 7]]);
    let virtual_address = u32::from_le_bytes([data[offset_header + 8], data[offset_header + 9], data[offset_header + 10], data[offset_header + 11]]);
    let physical_address = u32::from_le_bytes([data[offset_header + 12], data[offset_header + 13], data[offset_header + 14], data[offset_header + 15]]);
    let file_size = u32::from_le_bytes([data[offset_header + 16], data[offset_header + 17], data[offset_header + 18], data[offset_header + 19]]);
    let memory_size = u32::from_le_bytes([data[offset_header + 20], data[offset_header + 21], data[offset_header + 22], data[offset_header + 23]]);
    let flags = u32::from_le_bytes([data[offset_header + 24], data[offset_header + 25], data[offset_header + 26], data[offset_header + 27]]);
    let align = u32::from_le_bytes([data[offset_header + 28], data[offset_header + 29], data[offset_header + 30], data[offset_header + 31]]);

    let header = Elf32ProgramHeader {segment_type, offset, virtual_address, physical_address, file_size, memory_size, flags, align};

    Ok(header)
}

pub fn parse_program_headers(data: &[u8], program_header_offset: u32, program_header_size: u16, program_header_count: u16) -> Result<(), &'static str>{ 
    
    let header_offset = program_header_offset as usize;
    let header_count = program_header_count as usize;
    let header_size = program_header_size as usize;

    let total_size = header_size * header_count;

    if header_offset + total_size > data.len() {
        return Err("Header table out of bounds")
    }
    
    let mut current_offset = header_offset;

    for _ in 0..header_count {
        let header = parse_program_header(data, current_offset)?;

        if header.segment_type == 1 {
            if header.memory_size < header.file_size {
                return Err("Invalid segment size")
            }

            if (header.offset as usize) + (header.file_size as usize) > data.len() {
                return Err("Segment data out of bounds")
            }
            
        }

        current_offset += header_size;
    }

    Ok(())

}

/*
+0   segment_type       4 байта
+4   offset             4 байта
+8   virtual_address    4 байта
+12  physical_address   4 байта
+16  file_size          4 байта
+20  memory_size        4 байта
+24  flags              4 байта
+28  align              4 байта

 */