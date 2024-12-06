use crate::decode_instruction::{DecodedInstruction, Opcode, Register};
use crate::vm::VM;

pub(crate) fn execute_instruction(vm: &mut VM, instruction: DecodedInstruction) {
    match instruction.opcode {
        // R Type Instructions
        Opcode::Add => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) + vm.reg(instruction.rs2)
        }
        Opcode::Sub => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) - vm.reg(instruction.rs2)
        }
        Opcode::Xor => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) ^ vm.reg(instruction.rs2)
        }
        Opcode::Or => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) | vm.reg(instruction.rs2)
        }
        Opcode::And => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) & vm.reg(instruction.rs2)
        }
        Opcode::Sll => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) << vm.reg(instruction.rs2)
        }
        Opcode::Srl => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) >> vm.reg(instruction.rs2)
        }
        Opcode::Sra => {
            // TODO: deal with sign
            unimplemented!()
        }
        Opcode::Slt => {
            // TODO: deal with sign
            unimplemented!()
        }
        Opcode::Sltu => {
            unimplemented!()
        }

        // I Arithmetic Instructions
        Opcode::Addi => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
        }
        Opcode::Xori => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) ^ instruction.imm;
        }
        Opcode::Ori => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) | instruction.imm;
        }
        Opcode::Andi => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) & instruction.imm;
        }
        Opcode::Slli => {
            unimplemented!()
        }
        Opcode::Srli => {
            unimplemented!()
        }
        Opcode::Srai => {
            unimplemented!()
        }
        Opcode::Slti => {
            unimplemented!()
        }
        Opcode::Sltiu => {
            unimplemented!()
        }

        // I Memory Instructions
        Opcode::Lb => {}
        Opcode::Lh => {}
        Opcode::Lw => {}
        Opcode::Lbu => {}
        Opcode::Lhu => {}

        // Store Instructions
        Opcode::Sb => {}
        Opcode::Sh => {}
        Opcode::Sw => {}

        // Branch Instructions
        Opcode::Beq => {}
        Opcode::Bne => {}
        Opcode::Blt => {}
        Opcode::Bge => {}
        Opcode::Bltu => {}
        Opcode::Bgeu => {}

        // Jump Instructions
        Opcode::Jal => {}
        Opcode::Jalr => {}

        Opcode::Lui => {}
        Opcode::Auipc => {}

        // System Instructions
        Opcode::Ecall => {
            let function = vm.reg(Register::A7 as u32);
            match function {
                93 => {
                    // HALT
                    let exit_code = vm.reg(Register::A0 as u32);
                    vm.halted = true;
                    vm.exit_code = exit_code;
                }
                _ => unimplemented!(),
            }
        }
        Opcode::Ebreak => {}
        Opcode::Eother => {}
        Opcode::Fence => {}
    }
}
