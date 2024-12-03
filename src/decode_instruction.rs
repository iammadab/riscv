struct DecodedInstruction {
    opcode: u32
}

fn decode_instruction(instruction: u32) -> DecodedInstruction {
    todo!()
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
