use crate::decode_instruction::decode_instruction;
use crate::elf::{parse_elf, u32_le, ProgramInfo};
use crate::execute_instruction::execute_instruction;

// TODO: consider using paged memory
pub(crate) struct VM {
    registers: [u32; 32],
    memory: Vec<u8>,
    pub(crate) pc: u32,
    pub(crate) halted: bool,
    pub(crate) exit_code: u32,

    blackhole: u32,
}

impl VM {
    fn init() -> Self {
        Self {
            registers: [0; 32],
            memory: vec![0; 1 << 32],
            pc: 0,
            halted: false,
            exit_code: 0,
            blackhole: 0,
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
            registers: [0; 32],
            memory,
            pc: program.entry_point,
            halted: false,
            exit_code: 0,
            blackhole: 0,
        }
    }

    pub(crate) fn reg(&self, addr: u32) -> u32 {
        self.registers[addr as usize]
    }

    pub(crate) fn reg_mut(&mut self, addr: u32) -> &mut u32 {
        if addr == 0 {
            &mut self.blackhole
        } else {
            &mut self.registers[addr as usize]
        }
    }

    pub(crate) fn mem(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn mem_mut(&mut self, addr: u32) -> &mut u8 {
        &mut self.memory[addr as usize]
    }

    pub(crate) fn mem32(&self, addr: u32) -> [u8; 4] {
        let addr = addr as usize;
        [
            self.memory[addr],
            self.memory[addr + 1],
            self.memory[addr + 2],
            self.memory[addr + 3],
        ]
    }

    fn load_instruction(&self, pc: u32) -> [u8; 4] {
        self.mem32(pc)
    }

    fn run(&mut self) {
        while !self.halted {
            // eprintln!("pc: {:x}", self.pc);

            // fetch instruction
            let instruction = self.load_instruction(self.pc);

            // decode instruction
            let decoded_instruction = decode_instruction(u32_le(&instruction));

            if decoded_instruction.is_err() {
                eprintln!("pc: {:x}", self.pc);
                eprintln!(
                    "halting due to unsupported instruction: {:b}",
                    u32_le(&instruction)
                );
                self.halted = true;
                self.exit_code = 1;
                break;
            }

            // dbg!(decoded_instruction.clone().unwrap().opcode);

            // execute instruction
            execute_instruction(self, decoded_instruction.unwrap());

            // eprintln!("registers: {:?}", self.registers);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decode_instruction::{DecodedInstruction, InstructionType, Opcode, Register};
    use crate::execute_instruction::execute_instruction;
    use crate::vm::VM;
    use std::fs;

    #[test]
    fn test_rv32ui() {
        let _ = fs::read_dir("e2e-tests")
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok())
            .map(|entry| run_test_elf(entry.path().to_str().unwrap().to_string()))
            .collect::<Vec<_>>();
    }

    fn run_test_elf(path: String) {
        let exclude = vec![
            "e2e-tests/rv32ui-p-lbu",
            "e2e-tests/rv32ui-p-sh",
            "e2e-tests/rv32ui-p-lhu",
            "e2e-tests/rv32ui-p-lh",
            "e2e-tests/rv32ui-p-lb",
            "e2e-tests/rv32ui-p-fence_i",
            "e2e-tests/rv32ui-p-sb",
            "e2e-tests/rv32ui-p-sw",
            "e2e-tests/rv32ui-p-ma_data",
            "e2e-tests/rv32ui-p-jalr",
        ];
        if exclude.contains(&path.as_str()) {
            return;
        }

        println!("running test: {}", path);

        let mut vm = VM::init_from_elf(path);
        vm.run();

        println!("exit-code: {}", vm.exit_code);
        assert!(vm.halted);
        assert_eq!(vm.exit_code, 0);
    }

    #[test]
    fn vm_halt_via_ecall() {
        let mut vm = VM::init();
        assert_eq!(vm.reg(Register::A7.into()), 0);

        // set the a7 register to 93
        let set_a7_insn = DecodedInstruction {
            inst_type: InstructionType::I,
            opcode: Opcode::Addi,
            rd: Register::A7 as u32,
            rs1: Register::Zero as u32,
            rs2: 0,
            funct3: 0,
            funct7: 0,
            imm: 93,
        };
        execute_instruction(&mut vm, set_a7_insn);
        assert_eq!(vm.reg(Register::A7.into()), 93);

        // set the a0 to exit code
        let set_a0_insn = DecodedInstruction {
            inst_type: InstructionType::I,
            opcode: Opcode::Addi,
            rd: Register::A0.into(),
            rs1: Register::Zero.into(),
            rs2: 0,
            funct3: 0,
            funct7: 0,
            imm: 4,
        };
        execute_instruction(&mut vm, set_a0_insn);
        assert_eq!(vm.reg(Register::A0.into()), 4);

        // trigger ecall
        assert_eq!(vm.halted, false);
        let ecall_insn = DecodedInstruction {
            inst_type: InstructionType::I,
            opcode: Opcode::Ecall,
            rd: 0,
            rs1: 0,
            rs2: 0,
            funct3: 0,
            funct7: 0,
            imm: 0,
        };
        execute_instruction(&mut vm, ecall_insn);

        // assert state
        assert_eq!(vm.halted, true);
        assert_eq!(vm.exit_code, 4);
    }
}
