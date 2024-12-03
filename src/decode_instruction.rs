#[derive(Debug)]
enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J,
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

    Ecall,
    Ebreak,
    Fence,
}

#[derive(Debug)]
pub(crate) struct DecodedInstruction {
    inst_type: InstructionType,
    opcode: Opcode,
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
        _ => panic!("unsupported instruction")
    };

    DecodedInstruction {
        inst_type,
        opcode: Opcode::Add,
    }
}

fn mask(n: u8) -> u32 {
    (1 << n) - 1
}

#[cfg(test)]
mod tests {
    use crate::decode_instruction::decode_instruction;

    #[test]
    fn test_instruction_decoding() {
        let instruction: u32 = 0x00c58533;
        dbg!(decode_instruction(instruction));
    }
}
