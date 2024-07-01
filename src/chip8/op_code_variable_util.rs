/// Get the last 12 bytes from a 16 byte operation code.
pub fn get_nnn(op_code: u16) -> u16 {
    op_code & 0x0FFF
}

/// Get the last 4 bytes from a 16 byte operation code.
pub fn get_nibble(op_code: u16) -> u16 {
    op_code & 0x000F
}

/// Get the last 4 bytes from the first 8 bytes of an operation code.
pub fn get_x(op_code: u16) -> u16 {
    (op_code & 0x0F00) >> 8
}

/// Get the first 4 bytes from the last 8 bytes of an operation code.
pub fn get_y(op_code: u16) -> u16 {
    (op_code & 0x00F0) >> 4
}

/// Get the last 8 bytes from a 16 byte operation code.
pub fn get_byte(op_code: u16) -> u16 {
    op_code & 0x00FF
}