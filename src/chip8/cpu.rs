use crate::chip8::op_code_variable_util::{get_byte, get_nibble, get_nnn, get_x, get_y};

use super::{font::FONT_SET, op_code_variable_util::split_op_code};

const MEMORY: usize = 4096;
const V_REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;
const VRAM_WIDTH: usize = 32;
const VRAM_HEIGHT: usize = 64;

// CHIP-8 Interpreter
pub struct CHIP8 {
    ram: [u8; MEMORY],
    pub vram: [u8; (VRAM_WIDTH * VRAM_HEIGHT)],
    v: [u8; V_REGISTER_COUNT],
    i: u16,
    vf: bool,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16, //program counter
    stack_pointer: u8,
    stack: [u16; STACK_SIZE]
}

impl CHIP8 {
    pub fn new() -> Self {
        // Load font data to the interpreter area (registers 0x000 to 0x1FF)
        let mut ram: [u8; MEMORY] = [0; MEMORY];
        for (index, font_char_piece) in FONT_SET.iter().enumerate() {
            ram[index] = *font_char_piece;
        }

        CHIP8 {
            ram,
            i: 0,
            v: [0; V_REGISTER_COUNT],
            vram: [0; (VRAM_WIDTH * VRAM_HEIGHT)],
            vf: false,
            delay_timer: 0,
            sound_timer: 0,
            pc: PROGRAM_START as u16,
            stack_pointer: 0,
            stack: [0; STACK_SIZE]
        }
    }

    /// Loads ROM data to the interpreter
    pub fn load_rom_data(&mut self, data: &[u8]) {
        if data.len() > (MEMORY - PROGRAM_START) {
            panic!("ERROR: Rom data exceeds {} bytes. Exiting...", MEMORY);
        }

        for (i, op_data) in data.iter().enumerate() {
            self.ram[PROGRAM_START + i] = *op_data;
        }
    }

    /// Get the current instruction and increase program counter to the next instruction
    pub fn run_next_instruction(&mut self) {
        let op_code: u16 = self.get_op_code();
        println!("[INSTRUCTION]: {:#06X}", op_code);
        match split_op_code(op_code) {
            (0x0, 0x0, 0xE, 0x0) => { println!("0x00E0 not implemented yet..."); }
            (0x6, _, _, _) => {
                // Set Vx = kk
                self.v[get_x(op_code) as usize] = get_byte(op_code) as u8;
            }
            (0x7, _, _, _) => {
                // Set Vx = Vx + kk
                let x: usize = get_x(op_code) as usize;
                let kk: u8 = get_byte(op_code) as u8;
                self.v[x] = self.v[x] + kk;
            }
            (0xA, _, _, _) => {
                // Set I = nnn
                self.i = get_nnn(op_code);
            }
            (0xD, _, _, _) => {
                let x: u16 = get_x(op_code);
                let y: u16 = get_y(op_code);
                let nibble: u16 = get_nibble(op_code);
                println!("{}, {}, {}", x, y, nibble);
                println!("0xDxyn not implemented yet...");
            }
            _ => { println!("Instruction not supported by CHIP-8."); }
        }

        self.next_instruction();
    }

    /// Get an operation code using the program counter
    fn get_op_code(&self) -> u16 {
        // Cast is required because u8 can't be indexed by u16
        let addr: usize = self.pc as usize;
        (self.ram[addr] as u16) << 8 | (self.ram[addr + 1] as u16)
    }

    // Increases the program counter by 2 to go to the next program instruction
    fn next_instruction(&mut self) {
        self.pc += 2;
    }
}
