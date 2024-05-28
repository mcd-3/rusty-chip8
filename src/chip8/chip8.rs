const MEMORY: usize = 4096;

// CHIP-8 Interpreter
pub struct CHIP8 {
    v: [u8],
    i: [u16],
    vf: boolean,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u8,
    stack_pointer: u8,
    stack: [u16]
}

impl CHIP8 {
    fn new(
        v: [u8],
        i: [u16],
        vf: boolean,
        delay_timer: u8,
        sound_timer: u8,
        program_counter: u8,
        stack_pointer: u8,
        stack: [u16]
    ) -> Self {
        CHIP8 {

        }
    }

    fn decrement_sound_timer() {

    }


}