use super::font::FONT_SET;

const MEMORY: usize = 4096;
const V_REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;

// CHIP-8 Interpreter
pub struct CHIP8 {
    ram: [u8; MEMORY],
    v: [u8; V_REGISTER_COUNT],
    i: u16,
    vf: bool,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; STACK_SIZE]
}

impl CHIP8 {
    pub fn new() -> Self {
        // Load program data at 0x200
        let mut ram: [u8; MEMORY] = [0; MEMORY];

        CHIP8 {
            ram,
            i: 0,
            v: [0; V_REGISTER_COUNT],
            vf: false,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE]
        }
    }

    pub fn decrement_sound_timer(&self) {
        println!("Printing font set...");
        println!("{}", FONT_SET[0]);
        println!("{}", self.delay_timer);
    }

}