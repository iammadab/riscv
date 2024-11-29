use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};

const MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

struct HeaderInfo {
    entry_point: u32,
    program_header_table_offset: u32,
    // TODO: can this be u16?
    program_header_entry_size: u32,
    program_entry_count: u32
}

// TODO: should return the entry, code content + location, data content + location, pc
fn parse_elf(file_path: String) {
    let mut f = BufReader::new(File::open(file_path).unwrap());

    parse_elf_header(&mut f);

    // grab all program header contents
    // parse program header

    todo!()
}

fn parse_elf_header(f: &mut BufReader<File>) -> HeaderInfo {
    // TODO: add better documentation
    // TODO: remove unwraps

    // parse elf header
    // must be elf binary
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
    let program_entry_count= u32_le(&read_bytes::<2>(f).unwrap());

    HeaderInfo {
        entry_point,
        program_header_table_offset,
        program_header_entry_size,
        program_entry_count
    }
}

fn read_bytes<const N: usize>(f: &mut BufReader<File>) -> io::Result<[u8; N]> {
    let mut buffer = [0_u8; N];
    f.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn seek(f: &mut BufReader<File>, offset_from_start: u64) -> io::Result<u64> {
    f.seek(SeekFrom::Start(offset_from_start))
}

fn u32_le(data: &[u8]) -> u32 {
    let mut buffer = [0u8; 4];
    let len = data.len().min(4);
    buffer[..len].copy_from_slice(&data[..len]);
    u32::from_le_bytes(buffer)
}
#[cfg(test)]
mod test {
    use crate::elf::parse_elf_header;
    use std::fs::File;
    use std::io::BufReader;

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
