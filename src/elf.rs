use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

const MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

// TODO: do proper error handling

fn parse_elf(file_path: String) {
    // TODO: add better documentation

    let mut f = BufReader::new(File::open(file_path).unwrap());

    // parse elf header

    // verify_magic_number
    let file_magic_number: [u8; 4] = read_bytes(&mut f).unwrap();
    assert_eq!(file_magic_number, MAGIC_NUMBER);


    // should return the entry, code content + location, data content + location, pc
    // verify constraints
    // serial then use seeks to jump offsets
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