use crate::decode_instruction::decode_instruction;
use crate::elf::{parse_elf, u32_le, ProgramInfo};

// TODO: consider using paged memory
struct VM {
    registers: [u32; 33],
    memory: Vec<u8>,
    pc: u32,
}

impl VM {
    fn init(program: ProgramInfo) -> Self {
        let mut memory = vec![0; 1 << 32];

        // load code
        let code_start = program.code.0 as usize;
        let code_end = code_start + program.code.1.len();
        memory[code_start..code_end].copy_from_slice(&program.code.1);

        // load data
        let data_start = program.data.0 as usize;
        let data_end = data_start + program.data.1.len();
        memory[data_start..data_end].copy_from_slice(&program.data.1);

        Self {
            registers: [0; 33],
            memory,
            pc: program.entry_point,
        }
    }

    fn init_from_elf(path: String) -> Self {
        let program_info = parse_elf(path);
        Self::init(program_info)
    }

    fn reg(&self, addr: u32) -> u32 {
        self.registers[addr as usize]
    }

    fn reg_mut(&mut self, addr: u32) -> &mut u32 {
        &mut self.registers[addr as usize]
    }

    fn mem(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_mut(&mut self, addr: u32) -> &mut u8 {
        &mut self.memory[addr as usize]
    }

    fn load_instruction(&self, addr: u32) -> [u8; 4] {
        let pc = addr as usize;
        [
            self.memory[pc],
            self.memory[pc + 1],
            self.memory[pc + 2],
            self.memory[pc + 3],
        ]
    }

    fn run(&mut self) {
        loop {
            // fetch instruction
            let instruction = self.load_instruction(self.pc);

            // decode instruction
            let decoded_instruction = decode_instruction(u32_le(&instruction));
            dbg!(decoded_instruction);

            // execute instruction

            // update pc
            self.pc += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::VM;

    #[test]
    fn fake_test() {
        let mut vm = VM::init_from_elf("test-data/rv32ui-p-add".to_string());
        vm.run();
    }
}
