use crate::decode_instruction::{mask, sext, DecodedInstruction, Opcode, Register};
use crate::vm::VM;

pub(crate) fn execute_instruction(vm: &mut VM, instruction: DecodedInstruction) {
    match instruction.opcode {
        // R Type Instructions
        Opcode::Add => {
            *vm.reg_mut(instruction.rd) = vm
                .reg(instruction.rs1)
                .wrapping_add(vm.reg(instruction.rs2))
        }
        Opcode::Sub => {
            *vm.reg_mut(instruction.rd) = vm
                .reg(instruction.rs1)
                .wrapping_sub(vm.reg(instruction.rs2));
        }
        Opcode::Xor => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) ^ vm.reg(instruction.rs2);
        }
        Opcode::Or => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) | vm.reg(instruction.rs2);
        }
        Opcode::And => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) & vm.reg(instruction.rs2);
        }
        Opcode::Sll => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) << vm.reg(instruction.rs2);
        }
        Opcode::Srl => {
            *vm.reg_mut(instruction.rd) =
                vm.reg(instruction.rs1) >> (vm.reg(instruction.rs2) & mask(5));
        }
        Opcode::Sra => {
            let shift = vm.reg(instruction.rs2) & mask(5);
            *vm.reg_mut(instruction.rd) =
                sext(vm.reg(instruction.rs1) >> shift, 32 - shift as usize);
        }
        Opcode::Slt => {
            *vm.reg_mut(instruction.rd) =
                if (vm.reg(instruction.rs1) as i32) < vm.reg(instruction.rs2) as i32 {
                    1
                } else {
                    0
                }
        }
        Opcode::Sltu => {
            *vm.reg_mut(instruction.rd) = if vm.reg(instruction.rs1) < vm.reg(instruction.rs2) {
                1
            } else {
                0
            }
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
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) << (instruction.imm & mask(5));
        }
        Opcode::Srli => {
            *vm.reg_mut(instruction.rd) = vm.reg(instruction.rs1) >> (instruction.imm & mask(5));
        }
        Opcode::Srai => {
            let shift = instruction.imm & mask(5);
            *vm.reg_mut(instruction.rd) =
                sext(vm.reg(instruction.rs1) >> shift, 32 - shift as usize);
        }
        Opcode::Slti => {
            *vm.reg_mut(instruction.rd) =
                if (vm.reg(instruction.rs1) as i32) < instruction.imm as i32 {
                    1
                } else {
                    0
                }
        }
        Opcode::Sltiu => {
            *vm.reg_mut(instruction.rd) = if vm.reg(instruction.rs1) < instruction.imm {
                1
            } else {
                0
            }
        }

        // I Memory Instructions
        Opcode::Lb => {
            unimplemented!()
        }
        Opcode::Lh => {
            unimplemented!()
        }
        Opcode::Lw => {
            unimplemented!()
        }
        Opcode::Lbu => {
            unimplemented!()
        }
        Opcode::Lhu => {
            unimplemented!()
        }

        // Store Instructions
        Opcode::Sb => {
            unimplemented!()
        }
        Opcode::Sh => {
            unimplemented!()
        }
        Opcode::Sw => {
            unimplemented!()
        }

        // Branch Instructions
        Opcode::Beq => {
            if vm.reg(instruction.rs1) == vm.reg(instruction.rs2) {
                vm.pc = vm.pc.wrapping_add(instruction.imm);
                return;
            }
        }
        Opcode::Bne => {
            if vm.reg(instruction.rs1) != vm.reg(instruction.rs2) {
                vm.pc = vm.pc.wrapping_add(instruction.imm);
                return;
            }
        }
        Opcode::Blt => {
            if (vm.reg(instruction.rs1) as i32) < (vm.reg(instruction.rs2) as i32) {
                vm.pc = vm.pc.wrapping_add(instruction.imm);
                return;
            }
        }
        Opcode::Bge => {
            if (vm.reg(instruction.rs1) as i32) >= (vm.reg(instruction.rs2) as i32) {
                vm.pc = vm.pc.wrapping_add(instruction.imm);
                return;
            }
        }
        Opcode::Bltu => {
            if vm.reg(instruction.rs1) < vm.reg(instruction.rs2) {
                vm.pc = vm.pc.wrapping_add(instruction.imm);
                return;
            }
        }
        Opcode::Bgeu => {
            if vm.reg(instruction.rs1) >= vm.reg(instruction.rs2) {
                vm.pc = vm.pc.wrapping_add(instruction.imm);
                return;
            }
        }

        // Jump Instructions
        Opcode::Jal => {
            *vm.reg_mut(instruction.rd) = vm.pc.wrapping_add(4);
            vm.pc = vm.pc.wrapping_add(instruction.imm);
            return;
        }
        Opcode::Jalr => {
            unimplemented!()
        }

        Opcode::Lui => *vm.reg_mut(instruction.rd) = instruction.imm,
        Opcode::Auipc => *vm.reg_mut(instruction.rd) = vm.pc.wrapping_add(instruction.imm),

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
                _ => {
                    eprintln!("skipping ecall a7 reg: {}", function);
                }
            }
        }
        Opcode::Ebreak => {
            unimplemented!()
        }
        Opcode::Eother => {
            // skipping execution of this instruction
        }
        Opcode::Fence => {
            // skipping execution of this instruction
        }
    }

    // update pc
    vm.pc += 4;
}
