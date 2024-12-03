use crate::decode_instruction::decode_instruction;
use crate::elf::{ProgramInfo, u32_le};

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

    fn reg(&self, addr: u32) -> u32 {
        self.registers[addr as usize]
    }

    fn reg_mut(&mut self, addr: u32) -> &mut u32 {
        &mut self.registers[addr as usize]
    }

    fn mem(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_mut(&mut self, addr: u32) -> &mut u8{
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
            // TODO: only care about the opcode for now
            decode_instruction(u32_le(&instruction));

            // execute instruction
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fake_test() {
        todo!()
    }
}