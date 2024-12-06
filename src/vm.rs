use crate::decode_instruction::decode_instruction;
use crate::elf::{parse_elf, u32_le, ProgramInfo};
use crate::execute_instruction::execute_instruction;

// TODO: consider using paged memory
pub(crate) struct VM {
    registers: [u32; 33],
    memory: Vec<u8>,
    pc: u32,
    pub(crate) halted: bool,
    pub(crate) exit_code: u32,
}

impl VM {
    fn init() -> Self {
        Self {
            registers: [0; 33],
            memory: vec![0; 1 << 32],
            pc: 0,
            halted: false,
            exit_code: 0
        }
    }

    fn init_from_elf(path: String) -> Self {
        let program = parse_elf(path);

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
            halted: false,
            exit_code: 0,
        }
    }

    pub(crate) fn reg(&self, addr: u32) -> u32 {
        self.registers[addr as usize]
    }

    pub(crate) fn reg_mut(&mut self, addr: u32) -> &mut u32 {
        &mut self.registers[addr as usize]
    }

    pub(crate) fn mem(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn mem_mut(&mut self, addr: u32) -> &mut u8 {
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

            if decoded_instruction.is_err() {
                self.halted = true;
                self.exit_code = 1;
                break;
            }

            // execute instruction
            execute_instruction(self, decoded_instruction.unwrap());

            // update pc
            self.pc += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decode_instruction::{DecodedInstruction, InstructionType, Opcode};
    use crate::vm::VM;

    #[test]
    fn fake_test() {
        let mut vm = VM::init_from_elf("test-data/rv32ui-p-add".to_string());
        vm.run();
    }

    #[test]
    fn vm_halt_via_ecall() {
        let mut vm = VM::init();
        // set the a7 register to 93
        let set_a7_insn = DecodedInstruction {
            inst_type: InstructionType::I,
            opcode: Opcode::Addi,
            rd: A7,
            rs1: 0,
            rs2: 0,
            funct3: 0,
            funct7: 0,
            imm: 0,
        }
        // set the a0 to exit code
        // trigger ecall
        // assert state
    }
}
