enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J
}

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
    Fence
}

struct DecodedInstruction {
    inst_type: InstructionType,
    opcode: Opcode
}

fn decode_instruction(instruction: u32) -> DecodedInstruction {
    let opcode_value = instruction & mask(6);
    todo!()
}

fn mask(n: u8) -> u32 {
    (1 << n) - 1
}

#[test]
mod tests {
    use crate::decode_instruction::decode_instruction;

    #[test]
    fn test_instruction_decoding() {
        let instruction: u32 = 0x00c58533;
        decode_instruction(instruction);
    }
}
