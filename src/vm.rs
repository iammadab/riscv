struct VM {
    registers: [u8; 33],
    memory: Vec<u8>,
}

impl VM {
    fn init() -> Self {
        Self {
            registers: [0; 33],
            memory: vec![0; 1 << 32],
        }
    }
}
