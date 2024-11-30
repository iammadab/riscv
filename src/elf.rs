use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};

const MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

struct ElfHeaderInfo {
    entry_point: u32,
    program_header_table_offset: u32,
    program_header_entry_size: u32,
    program_entry_count: u32,
}

struct ProgramHeaderInfo {
    data: Vec<u8>,
    virtual_address: u32,
    // true = code_segment, false = data_segment
    code: bool,
}

// TODO: should return the entry, code content + location, data content + location, pc
fn parse_elf(file_path: String) {
    let mut f = BufReader::new(File::open(file_path).unwrap());

    let header_info = parse_elf_header(&mut f);

    // if anything was wrong, the parse_elf_header would have panicked
    // at this point we should know the entry point
    // what do we return for this case?
    // - entry point
    // - code segment
    // - data segment

    // how do I determine the different segments from the elf file?
    // I need to parse the flag
    // assuming I can parse the program header, how do I choose what to return
    // I can have a mutable for code and data, option and reset based on that
    // with some requirement that there must always exist code and data??

    // not sure what the type will be yet tho
    // let mut code = None;
    // let mut data = None;

    for i in 0..header_info.program_entry_count {
        let offset =
            (i * header_info.program_header_entry_size) + header_info.program_header_table_offset;
        let program_header = parse_program_header(&mut f, offset);
    }

    // grab all program header contents
    // parse program header

    todo!()
}

fn parse_elf_header(f: &mut BufReader<File>) -> ElfHeaderInfo {
    // verify_magic_number
    let file_magic_number: [u8; 4] = read_bytes(f).unwrap();
    assert_eq!(file_magic_number, MAGIC_NUMBER);

    // the class must be 32 bits
    assert_eq!(read_bytes(f).unwrap(), [0x01]);

    // ensure little-endian
    assert_eq!(read_bytes(f).unwrap(), [0x01]);

    // skip to offset 0x10 -> e_type
    seek(f, 0x10).unwrap();

    // ensure file type is executable
    assert_eq!(read_bytes(f).unwrap(), [0x02]);

    // seek to machine type
    seek(f, 0x12).unwrap();

    // ensure machine type is riscv (0xF3)
    assert_eq!(read_bytes(f).unwrap(), [0xF3]);

    // seek to entry point
    seek(f, 0x18).unwrap();

    // extract entry point
    let entry_point = u32_le(&read_bytes::<4>(f).unwrap());

    // extract program header table offset
    let program_header_table_offset = u32_le(&read_bytes::<4>(f).unwrap());

    // seek to program header size
    seek(f, 0x2A).unwrap();

    // extract program header size
    let program_header_entry_size = u32_le(&read_bytes::<2>(f).unwrap());

    // extract program header count
    let program_entry_count = u32_le(&read_bytes::<2>(f).unwrap());

    ElfHeaderInfo {
        entry_point,
        program_header_table_offset,
        program_header_entry_size,
        program_entry_count,
    }
}

fn parse_program_header(f: &mut BufReader<File>, offset: u32) -> Option<ProgramHeaderInfo> {
    // seek to offset
    seek(f, offset).unwrap();

    // read type
    let p_type = u32_le(&read_bytes::<4>(f).unwrap());

    // ensure program header is of type LOAD
    if p_type != 1 {
        return None;
    }

    let p_offset = u32_le(&read_bytes::<4>(f).unwrap());
    let virtual_address = u32_le(&read_bytes::<4>(f).unwrap());

    // seek to p_filesz
    seek(f, offset + 0x10).unwrap();

    let p_filesz = u32_le(&read_bytes::<4>(f).unwrap());
    let p_memsz = u32_le(&read_bytes::<4>(f).unwrap());
    let p_flags = u32_le(&read_bytes::<4>(f).unwrap());

    // seek to p_offset
    seek(f, p_offset).unwrap();

    // read header body
    let mut header_body = vec![0_u8; p_filesz as usize];
    f.read_exact(&mut header_body).unwrap();

    // decode flag
    // EXECUTABLE (E) = 1, WRITEABLE (W) = 2, READABLE (R) = 4
    // code = R + E = 4 + 1 = 5
    let is_code = if p_flags == 5 {
        true
    } else if p_flags == 6 {
        false
    } else {
        // neither code nor data panic
        panic!("expected code or data program header body");
    };

    Some(ProgramHeaderInfo {
        data: header_body,
        virtual_address,
        code: is_code,
    })
}

fn read_bytes<const N: usize>(f: &mut BufReader<File>) -> io::Result<[u8; N]> {
    let mut buffer = [0_u8; N];
    f.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn seek(f: &mut BufReader<File>, offset_from_start: u32) -> io::Result<u64> {
    f.seek(SeekFrom::Start(offset_from_start as u64))
}

fn u32_le(data: &[u8]) -> u32 {
    let mut buffer = [0u8; 4];
    let len = data.len().min(4);
    buffer[..len].copy_from_slice(&data[..len]);
    u32::from_le_bytes(buffer)
}
#[cfg(test)]
mod test {
    use crate::elf::{parse_elf, parse_elf_header};
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn fake_test() {
        parse_elf("test-data/rv32ui-p-add".to_string());
    }

    #[test]
    fn elf_header_parsing() {
        let mut f = BufReader::new(File::open("test-data/rv32ui-p-add").unwrap());
        let header_info = parse_elf_header(&mut f);
        assert_eq!(header_info.entry_point, 0x80000000);
        assert_eq!(header_info.program_header_table_offset, 0x34);
        assert_eq!(header_info.program_header_entry_size, 32);
        assert_eq!(header_info.program_entry_count, 3);
    }
}
