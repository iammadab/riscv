use crate::elf::ProgramInfo;

// TODO: consider using paged memory
struct VM {
    registers: [u32; 33],
    memory: Vec<u8>,
    pc: u32,
}

impl VM {
    fn init(&mut self, program: ProgramInfo) -> Self {
        let mut memory = vec![0; 1 << 32];

        // load code
        let code_start = program.code.0 as usize;
        memory[code_start..program.code.1.len()].copy_from_slice(&program.code.1);

        // load data
        let data_start = program.data.0 as usize;
        memory[data_start..program.data.1.len()].copy_from_slice(&program.data.1);

        Self {
            registers: [0; 33],
            memory,
            pc: program.entry_point,
        }
    }
}
