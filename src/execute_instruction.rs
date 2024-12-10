use crate::decode_instruction::{mask, sext, DecodedInstruction, Opcode, Register};
use crate::elf::u32_le;
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
            *vm.reg_mut(instruction.rd) =
                vm.reg(instruction.rs1) << (vm.reg(instruction.rs2) & mask(5));
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

        // Load Instructions
        Opcode::Lb => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let mem_data = u32_le(&vm.mem32(mem_addr));
            let mem_half_data = sext(mem_data & mask(8), 8);
            *vm.reg_mut(instruction.rd) = mem_half_data;
        }
        Opcode::Lh => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let mem_data = u32_le(&vm.mem32(mem_addr));
            let mem_half_data = sext(mem_data & mask(16), 16);
            *vm.reg_mut(instruction.rd) = mem_half_data;
        }
        Opcode::Lw => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            *vm.reg_mut(instruction.rd) = u32_le(&vm.mem32(mem_addr));
        }
        Opcode::Lbu => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let mem_data = u32_le(&vm.mem32(mem_addr));
            let mem_half_data = mem_data & mask(8);
            *vm.reg_mut(instruction.rd) = mem_half_data;
        }
        Opcode::Lhu => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let mem_data = u32_le(&vm.mem32(mem_addr));
            let mem_half_data = mem_data & mask(16);
            *vm.reg_mut(instruction.rd) = mem_half_data;
        }

        // Store Instructions
        Opcode::Sb => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let reg_data = vm.reg(instruction.rs2).to_le_bytes();
            *vm.mem_mut(mem_addr) = reg_data[0];
        }
        Opcode::Sh => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let reg_data = vm.reg(instruction.rs2).to_le_bytes();
            for i in 0..2 {
                *vm.mem_mut(mem_addr + i) = reg_data[i as usize];
            }
        }
        Opcode::Sw => {
            let mem_addr = vm.reg(instruction.rs1).wrapping_add(instruction.imm);
            let reg_data = vm.reg(instruction.rs2).to_le_bytes();
            for i in 0..4 {
                *vm.mem_mut(mem_addr + i) = reg_data[i as usize];
            }
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
            let rs1_value = vm.reg(instruction.rs1);
            *vm.reg_mut(instruction.rd) = vm.pc.wrapping_add(4);
            vm.pc = rs1_value.wrapping_add(instruction.imm);
            return;
        }

        Opcode::Lui => *vm.reg_mut(instruction.rd) = instruction.imm,
        Opcode::Auipc => *vm.reg_mut(instruction.rd) = vm.pc.wrapping_add(instruction.imm),

        // System Instructions
        Opcode::Ecall => {
            let function = vm.reg(Register::A7 as u32);
            match function {
                1 => {
                    // write register as decimal
                    let file_descriptor = vm.reg(Register::A0 as u32);
                    let register_addr = vm.reg(Register::A1 as u32);
                    match file_descriptor {
                        1 => println!("{}", register_addr),
                        2 => eprintln!("{}", register_addr),
                        _ => panic!("invalid file descriptor to print"),
                    }
                }
                64 => {
                    // write string
                    let file_descriptor = vm.reg(Register::A0 as u32);
                    let addr = vm.reg(Register::A1 as u32) as usize;
                    let len = vm.reg(Register::A2 as u32) as usize;
                    let bytes: &[u8] = &vm.memory[addr..(addr + len)];
                    let to_print =
                        String::from_utf8(bytes.to_vec()).expect("invalid print argument");
                    match file_descriptor {
                        1 => println!("{}", to_print),
                        2 => eprintln!("{}", to_print),
                        _ => panic!("invalid file descriptor for print"),
                    };
                }
                93 => {
                    // halt
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
    vm.pc = vm.pc.wrapping_add(4);
}
