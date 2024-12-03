#[derive(Debug)]
enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J,
    Fence,
}

#[derive(Debug)]
enum Opcode {
    Add,
    Sub,
    Xor,
    Or,
    And,
    Sll,
    Srl,
    Sra,
    Slt,
    Sltu,

    Addi,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Slti,
    Sltiu,

    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,

    Sb,
    Sh,
    Sw,

    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,

    Jal,
    Jalr,

    Lui,
    Auipc,

    Eany,
    Fence,
}

#[derive(Debug)]
pub(crate) struct DecodedInstruction {
    inst_type: InstructionType,
    opcode: Opcode,
    rd: u32,
    rs1: u32,
    rs2: u32,
    funct3: u32,
    funct7: u32,
    imm: u32,
}

pub(crate) fn decode_instruction(instruction: u32) -> DecodedInstruction {
    let opcode_value = instruction & mask(7);

    let inst_type = match opcode_value {
        0b0110011 => InstructionType::R,
        0b0010011 | 0b0000011 | 0b1110011 | 0b1100111 => InstructionType::I,
        0b0100011 => InstructionType::S,
        0b1100011 => InstructionType::B,
        0b1101111 => InstructionType::J,
        0b0110111 | 0b0010111 => InstructionType::U,
        0b0001111 => InstructionType::Fence,
        _ => panic!("unsupported instruction"),
    };

    let rd = (instruction >> 7) & mask(5);
    let rs1 = (instruction >> 15) & mask(5);
    let rs2 = (instruction >> 20) & mask(5);
    let funct3 = (instruction >> 12) & mask(3);
    let funct7 = (instruction >> 25) & mask(7);

    let imm = decode_immediate(&inst_type, instruction);

    DecodedInstruction {
        opcode: decode_opcode(opcode_value, &inst_type, funct3, funct7, imm),
        inst_type,
        rd,
        rs1,
        rs2,
        funct3,
        funct7,
        imm,
    }
}

fn decode_opcode(
    opcode_value: u32,
    inst_type: &InstructionType,
    funct3: u32,
    funct7: u32,
    imm: u32,
) -> Opcode {
    match inst_type {
        InstructionType::R => match funct3 {
            0x0 => match funct7 {
                0x00 => Opcode::Add,
                0x20 => Opcode::Sub,
                _ => panic!("unknown opcode"),
            },
            0x4 => Opcode::Xor,
            0x6 => Opcode::Or,
            0x7 => Opcode::And,
            0x1 => Opcode::Sll,
            0x5 => match funct7 {
                0x00 => Opcode::Srl,
                0x20 => Opcode::Sra,
                _ => panic!("unknown opcode"),
            },
            0x2 => Opcode::Slt,
            0x3 => Opcode::Sltu,
            _ => panic!("unknown opcode"),
        },
        InstructionType::I => {
            match opcode_value {
                // alu
                0b0010011 => match funct3 {
                    0x0 => Opcode::Addi,
                    0x4 => Opcode::Xori,
                    0x6 => Opcode::Ori,
                    0x7 => Opcode::Andi,
                    0x1 => Opcode::Slli,
                    0x5 => match (imm >> 5) & mask(7) {
                        0x00 => Opcode::Srli,
                        0x20 => Opcode::Srai,
                        _ => panic!("unknown opcode"),
                    },
                    0x2 => Opcode::Slti,
                    0x3 => Opcode::Sltiu,
                    _ => panic!("unknown opcode"),
                },
                // load
                0b0000011 => match funct3 {
                    0x0 => Opcode::Lb,
                    0x1 => Opcode::Lh,
                    0x2 => Opcode::Lw,
                    0x4 => Opcode::Lbu,
                    0x5 => Opcode::Lhu,
                    _ => panic!("unknown opcode"),
                },
                0b1100111 => Opcode::Jalr,
                0b1110011 => Opcode::Eany,
                _ => panic!("unknown opcode"),
            }
        }
        InstructionType::S => match funct3 {
            0x0 => Opcode::Sb,
            0x1 => Opcode::Sh,
            0x2 => Opcode::Sw,
            _ => panic!("unknown opcode"),
        },
        InstructionType::B => match funct3 {
            0x0 => Opcode::Beq,
            0x1 => Opcode::Bne,
            0x4 => Opcode::Blt,
            0x5 => Opcode::Bge,
            0x6 => Opcode::Bltu,
            0x7 => Opcode::Bgeu,
            _ => panic!("unknown opcode"),
        },
        InstructionType::U => match opcode_value {
            0b0110111 => Opcode::Lui,
            0b0010111 => Opcode::Auipc,
            _ => panic!("unknown opcode"),
        },
        InstructionType::J => Opcode::Jal,
        InstructionType::Fence => Opcode::Fence,
    }
}

fn decode_immediate(instruction_type: &InstructionType, instruction: u32) -> u32 {
    let mut imm = 0;
    match instruction_type {
        InstructionType::R | InstructionType::Fence => imm,
        InstructionType::I => {
            // inst[31:20] -> imm[11:0]
            sext(map_range(instruction, imm, 31, 11, 12), 12)
        }
        InstructionType::S => {
            // inst[11:7] -> imm[4:0]
            imm = map_range(instruction, imm, 11, 4, 5);
            // inst[31:25] -> imm[11:5]
            imm = map_range(instruction, imm, 31, 11, 7);
            // highest imm bit index = 11
            imm = sext(imm, 12);
            imm
        }
        InstructionType::B => {
            // inst[31] -> imm[12]
            imm = map_range(instruction, imm, 31, 12, 1);
            // inst[30:25] -> imm[10:5]
            imm = map_range(instruction, imm, 30, 10, 6);
            // inst[11:8] -> imm[4:1]
            imm = map_range(instruction, imm, 11, 4, 4);
            // inst[7] -> imm[11]
            imm = map_range(instruction, imm, 7, 11, 1);
            // highest imm bit index = 12
            imm = sext(imm, 13);
            imm
        }
        InstructionType::U => {
            // immediate for u type is supposed to be a 20 bit unsigned integer
            // during the operations lui and auipc, we are supposed to perform a 12 bit left shift
            // here we perform the left shift already.
            // note without left shift mapping will be inst[31:12] -> imm[19:0]

            // inst[31:12] -> imm[31:12]
            imm = map_range(instruction, imm, 31, 31, 20);
            // no need to sext already 32 bits
            imm
        }
        InstructionType::J => {
            // inst[31] -> imm[20]
            imm = map_range(instruction, imm, 31, 20, 1);
            // inst[30:21] -> imm[10:1]
            imm = map_range(instruction, imm, 30, 10, 10);
            // inst[20] -> imm[11]
            imm = map_range(instruction, imm, 20, 11, 1);
            // inst[19:12] -> imm[19:12]
            imm = map_range(instruction, imm, 19, 19, 8);
            // highest imm bit index = 20
            imm = sext(imm, 21);
            imm
        }
    }
}

/// Copies bit set in val_1 into some range in val_2
/// [31, ..., 4, 3, 2,  1, 0]
fn map_range(src: u32, dest: u32, src_start: u8, dest_start: u8, count: u8) -> u32 {
    let right_shift_value = src_start + 1 - count; // +1 because of 0 index
    let val_1_range = (src >> right_shift_value) & mask(count);

    let left_shift_value = dest_start + 1 - count; // +1 because of 0 index
    dest | (val_1_range << left_shift_value)
}

/// Sign Extension
/// extends a binary value of a certain bit count to a larger bit count (u16 in this case)
pub fn sext(val: u32, bit_count: usize) -> u32 {
    // if the sign bit is 1, add 1's to the most significant part of the number
    // NOTE: this does not change the 2's complement meaning

    // bit_count represent the original length of the sequence
    // right shift to erase all element other than first (bit_count - 1)
    let sign_bit = val >> (bit_count - 1);

    // if sign bit is a 1 (negative in 2's complement representation)
    // pad most significant side with 1's
    if sign_bit == 1 {
        // left shift by bit_count to prevent corruption of original bit values
        return val | (0xffffffff << bit_count);
    }

    // if not val already padded with 0's just return
    val
}

const fn mask(n: u8) -> u32 {
    (1 << n) - 1
}

#[cfg(test)]
mod tests {
    use crate::decode_instruction::{decode_immediate, decode_instruction, map_range};

    #[test]
    fn test_instruction_decoding() {
        let instruction: u32 = 0x00c58533;
        // TODO: fix this test
        dbg!(decode_instruction(instruction));
    }

    #[test]
    fn test_map_range() {
        let val: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let target_val: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            map_range(target_val, val, 31, 20, 8),
            0b0000_0000_0001_1111_1110_0000_0000_0000
        );

        let val: u32 = 0b0000_0000_0000_1111_1111_0000_0000_0000;

        assert_eq!(
            map_range(target_val, val, 31, 0, 1),
            0b0000_0000_0000_1111_1111_0000_0000_0001
        );
    }

    #[test]
    fn test_immediate_decoding() {
        // addi x10 x11 12 (I Type)
        assert_eq!(decode_instruction(0x00C58513).imm, 12);
        // sw x8, 6(x4) (S Type)
        assert_eq!(decode_instruction(0x00822323).imm, 6);
        // sw x8, -6(x4) (S Type)
        assert_eq!(decode_instruction(0xfe822d23).imm, -6_i32 as u32);
        // beq x5, x6, 20 (B Type)
        assert_eq!(decode_instruction(0x00628a63).imm, 20);
        // lui x5, 164 (U Type)
        assert_eq!(decode_instruction(0x000a42b7).imm >> 12, 164);
        // jal x5, 44 (J Type)
        assert_eq!(decode_instruction(0x02c002ef).imm, 44);
    }
}
