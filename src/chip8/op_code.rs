pub struct OpCode {
    op_code: u16
}

impl OpCode {
    pub fn new(op_code: u16) -> Self {
        OpCode {
            op_code
        }
    }

    /// Get the raw op_code
    pub fn get_instruction(&self) -> u16 {
        self.op_code
    }

    /// Get the last 12 bytes from a 16 byte operation code.
    pub fn get_nnn(&self) -> u16 {
        self.op_code & 0x0FFF
    }

    /// Get the last 4 bytes from a 16 byte operation code.
    pub fn get_nibble(&self) -> u16 {
        self.op_code & 0x000F
    }

    /// Get the last 4 bytes from the first 8 bytes of an operation code.
    pub fn get_x(&self) -> u16 {
        (self.op_code & 0x0F00) >> 8
    }

    /// Get the first 4 bytes from the last 8 bytes of an operation code.
    pub fn get_y(&self) -> u16 {
        (self.op_code & 0x00F0) >> 4
    }

    /// Get the last 8 bytes from a 16 byte operation code.
    pub fn get_byte(&self) -> u16 {
        self.op_code & 0x00FF
    }

    /// Split an opcode into a 4 sized tuple of u8.
    pub fn split_op_code(&self) -> (u8, u8, u8, u8) {
        let op_code_split: (u8, u8, u8, u8) = (
            ((self.op_code & 0xF000) >> 12) as u8,
            ((self.op_code & 0x0F00) >> 8) as u8,
            ((self.op_code & 0x00F0) >> 4) as u8,
            ((self.op_code & 0x000F)) as u8,
        );

        op_code_split
    }
}
