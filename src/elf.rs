use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};

const MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

fn parse_elf(file_path: String) {
    // TODO: add better documentation
    // TODO: should return the entry, code content + location, data content + location, pc
    // TODO: remove unwraps

    let mut f = BufReader::new(File::open(file_path).unwrap());

    // parse elf header

    // must be elf binary
    // verify_magic_number
    let file_magic_number: [u8; 4] = read_bytes(&mut f).unwrap();
    assert_eq!(file_magic_number, MAGIC_NUMBER);

    // the class must be 32 bits
    assert_eq!(read_bytes(&mut f).unwrap(), [0x01]);

    // TODO: handle endianess (skipping for now)
    //  determine effect

    // skip to offset 0x10 -> e_type
    f.seek(SeekFrom::Start(0x10)).unwrap();

    // ensure file type is executable
    assert_eq!(read_bytes(&mut f).unwrap(), [0x02]);

    // ensure machine type is riscv
    // ensure entry point address is not 0

    // extract the program header offset
    // extract the program header count
    // extract the program header len (bytes)

    // grab all program header contents
    // parse program header

    todo!()
}

fn read_bytes<const N: usize>(f: &mut BufReader<File>) -> io::Result<[u8; N]> {
    let mut buffer = [0_u8; N];
    f.read_exact(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod test {
    use crate::elf::parse_elf;

    #[test]
    fn elf_parsing() {
        parse_elf("test-data/rv32ui-p-add".to_string());
    }
}
