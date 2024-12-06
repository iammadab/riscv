use crate::decode_instruction::{DecodedInstruction, Opcode};
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
        Opcode::Addi => {}
        Opcode::Xori => {}
        Opcode::Ori => {}
        Opcode::Andi => {}
        Opcode::Slli => {}
        Opcode::Srli => {}
        Opcode::Srai => {}
        Opcode::Slti => {}
        Opcode::Sltiu => {}

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
        Opcode::Ecall => {}
        Opcode::Ebreak => {}
        Opcode::Eother => {}
        Opcode::Fence => {}
    }
}
