const MEMORY: usize = 4096;
const V_REGISTER_COUNT: u8 = 16;
const STACK_SIZE: u8 = 16;
const PROGRAM_START: usize = 0x200;

// CHIP-8 Interpreter
pub struct CHIP8 {
    ram: [u8; MEMORY],
    v: [u8; V_REGISTER_COUNT],
    i: u16,
    vf: boolean,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; STACK_SIZE]
}

impl CHIP8 {
    fn new(
        // v: [u8],
        // i: [u16],
        // vf: boolean,
        // delay_timer: u8,
        // sound_timer: u8,
        // program_counter: u8,
        // stack_pointer: u8,
        // stack: [u16]
    ) -> Self {
        CHIP8 {

        }
    }

    fn decrement_sound_timer() {

    }


}