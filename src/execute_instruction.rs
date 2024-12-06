use crate::decode_instruction::{DecodedInstruction, Opcode};
use crate::vm::VM;

pub(crate) fn execute_instruction(vm: &mut VM, instruction: DecodedInstruction) {
    match instruction.opcode {
        Opcode::Add => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) + vm.reg(instruction.rs2)
        }
        Opcode::Sub => {}
        Opcode::Xor => {}
        Opcode::Or => {}
        Opcode::And => {}
        Opcode::Sll => {}
        Opcode::Srl => {}
        Opcode::Sra => {}
        Opcode::Slt => {}
        Opcode::Sltu => {}
        Opcode::Addi => {}
        Opcode::Xori => {}
        Opcode::Ori => {}
        Opcode::Andi => {}
        Opcode::Slli => {}
        Opcode::Srli => {}
        Opcode::Srai => {}
        Opcode::Slti => {}
        Opcode::Sltiu => {}
        Opcode::Lb => {}
        Opcode::Lh => {}
        Opcode::Lw => {}
        Opcode::Lbu => {}
        Opcode::Lhu => {}
        Opcode::Sb => {}
        Opcode::Sh => {}
        Opcode::Sw => {}
        Opcode::Beq => {}
        Opcode::Bne => {}
        Opcode::Blt => {}
        Opcode::Bge => {}
        Opcode::Bltu => {}
        Opcode::Bgeu => {}
        Opcode::Jal => {}
        Opcode::Jalr => {}
        Opcode::Lui => {}
        Opcode::Auipc => {}
        Opcode::Ecall => {}
        Opcode::Ebreak => {}
        Opcode::Eother => {}
        Opcode::Fence => {}
    }
}
