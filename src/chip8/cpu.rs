use crate::chip8::op_code_variable_util::{get_byte, get_nibble, get_nnn, get_x, get_y};
use super::{font::FONT_SET, op_code_variable_util::split_op_code};

use rand;
use rand::Rng;

const MEMORY: usize = 4096;
const V_REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;
const VRAM_WIDTH: usize = 64;
const VRAM_HEIGHT: usize = 32;
const SPRITE_LENGTH: u8 = 5;
const TOTAL_KEYS: usize = 16;

// Sprites have 8 columns and can be up to 15 rows high
const SPRITE_WIDTH: u16 = 8;

// CHIP-8 Interpreter
pub struct CHIP8 {
    ram: [u8; MEMORY],
    pub vram: [u8; (VRAM_WIDTH * VRAM_HEIGHT)],
    v: [u8; V_REGISTER_COUNT],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16, //program counter
    stack_pointer: u8,
    stack: [u16; STACK_SIZE],
    keys: [bool; TOTAL_KEYS],
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
            delay_timer: 0,
            sound_timer: 0,
            pc: PROGRAM_START as u16,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; TOTAL_KEYS],
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

    /// Press a key on the 16-character keypad
    pub fn press_key(&mut self, key: usize, is_pressed: bool) {
        self.keys[key] = is_pressed;
    }

    /// Get the current instruction and increase program counter to the next instruction
    pub fn run_next_instruction(&mut self) {
        let op_code: u16 = self.get_op_code();
        println!("[INSTRUCTION]: {:#06X}", op_code);

        // TODO: Move this to a tick command
        // Both timers decrease at a rate of 60 hz, so they will
        //    need to be decoupled from a CPU cycle tick
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }

        // TODO: Move this to a tick command
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }


        match split_op_code(op_code) {
            (0x0, 0x0, 0xE, 0x0) => {
                // 00E0 - CLS
                // Clear the display
                for i in 0..self.vram.len() {
                    self.vram[i] = 0;
                }
                self.next_instruction();
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // 00EE - RET
                // Return from a subroutine
                self.pc = self.stack_pop();
                self.next_instruction();
            }
            (0x1, _, _, _) => {
                // 1nnn - JP addr
                // Jump to location nnn
                let nnn: u16 = get_nnn(op_code);
                self.jump_to_instruction(nnn);
            }
            (0x2, _, _, _) => {
                // 2nnn - CALL addr
                // Call subroutine at nnn
                let nnn = get_nnn(op_code);
                self.stack_push(self.pc);
                self.jump_to_instruction(nnn);
            }
            (0x3, _, _, _) => {
                let x: usize = get_x(op_code) as usize;
                let kk: u8 = get_byte(op_code) as u8;

                if self.v[x] == kk {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            (0x4, _, _, _) => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk
                let x = get_x(op_code);
                let kk = get_byte(op_code);
                if self.v[x as usize] != kk as u8 {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            (0x5, _, _, 0x0) => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                if self.v[x] == self.v[y] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            (0x6, _, _, _) => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk
                self.v[get_x(op_code) as usize] = get_byte(op_code) as u8;
                self.next_instruction();
            }
            (0x7, _, _, _) => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk
                let x: u16 = get_x(op_code) as u16;
                let kk: u16 = get_byte(op_code) as u16;

                let total: u16 = (self.v[x as usize] as u16) + (kk as u16);
                self.v[x as usize] = total as u8;

                self.next_instruction();
            }
            (0x8, _, _, 0x0) => {
                // 8xy0 - LD Vx, Vy
                // Set Vx = Vy
                let x = get_x(op_code);
                let y = get_y(op_code);
                self.v[x as usize] = self.v[y as usize];
                self.next_instruction();
            }
            (0x8, _, _, 0x1) => {
                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                self.v[x] |= self.v[y];

                // The original COSMAC VIP machine cleared VF
                self.cosmac_clear_vf();
                self.next_instruction();
            }
            (0x8, _, _, 0x2) => {
                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                self.v[x] &= self.v[y];

                // The original COSMAC VIP machine cleared VF
                self.cosmac_clear_vf();
                self.next_instruction();
            }
            (0x8, _, _, 0x3) => {
                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                self.v[x] ^= self.v[y];

                // The original COSMAC VIP machine cleared VF
                self.cosmac_clear_vf();
                self.next_instruction();
            }
            (0x8, _, _, 0x4) => {
                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                let total: u16 = self.v[x] as u16 + self.v[y] as u16;
                self.v[x] = total as u8;
                self.v[0x0f] = if total > 0xFF { 1 } else { 0 };

                self.next_instruction();
            }
            (0x8, _, _, 0x5) => {
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                let (vx, borrow) = self.v[x].overflowing_sub(self.v[y]);

                self.v[x] = vx;

                if borrow {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }

                self.next_instruction();
            }
            (0x8, _, _, 0x6) => {
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;

                // The original COSMAC VIP machine set VX to VY
                // This instruction is ignored in Super-CHIP and Chip-48
                self.cosmac_set_vx_to_vy(x, y);

                let shift: u8 = self.v[x] & 1;

                self.v[x] >>= 1;
                self.v[0xF] = shift;
                self.next_instruction();
            }
            (0x8, _, _, 0x7) => {
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);

                if self.v[y] > self.v[x] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }

                self.next_instruction();
            }
            (0x8, _, _, 0xE) => {
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                let x: usize = get_x(op_code) as usize;
                let y: usize = get_y(op_code) as usize;

                // The original COSMAC VIP machine set VX to VY
                // This instruction is ignored in Super-CHIP and Chip-48
                self.cosmac_set_vx_to_vy(x, y);

                let shift = (self.v[x] >> 7) & 1;

                self.v[x] <<= 1;
                self.v[0xF] = shift;
                self.next_instruction();
            }
            (0x9, _, _, 0x0) => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                let x: u16 = get_x(op_code);
                let y: u16 = get_y(op_code);
                if self.v[x as usize] != self.v[y as usize] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            (0xA, _, _, _) => {
                // Annn - LD I, addr
                // Set I = nnn
                self.i = get_nnn(op_code);
                self.next_instruction();
            }
            (0xB, _, _, _) => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                let nnn: u16 = get_nnn(op_code);
                self.jump_to_instruction(nnn + self.v[0] as u16)
            }
            (0xC, _, _, _) => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk
                let kk: u8 = get_byte(op_code) as u8;
                let x: u16 = get_x(op_code);
                let random_number: u8 = rand::thread_rng().gen_range(0..255);
                self.v[x as usize] = random_number & kk;
                self.next_instruction();
            }
            (0xD, _, _, _) => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let x: u16 = get_x(op_code);
                let y: u16 = get_y(op_code);
                let nibble: u16 = get_nibble(op_code);

                let vx = self.v[x as usize] as u16;
                let vy = self.v[y as usize] as u16;

                let mut bit_flipped = 0;

                for i in 0..nibble {
                    let address: u16 = self.i + i as u16;
                    let pixel: u8 = self.ram[address as usize];
                    for j in 0..SPRITE_WIDTH {
                        if (pixel & (0b1000_0000 >> j)) != 0 {
                            let x_coord: usize = (vx + j) as usize % VRAM_WIDTH;
                            let y_coord: usize = (vy + i) as usize % VRAM_HEIGHT;
                            let pixel_index = x_coord + VRAM_WIDTH * y_coord;
                            bit_flipped |= self.vram[pixel_index];
                            self.vram[pixel_index] ^= 1;
                        }
                    }
                }

                if bit_flipped == 1 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.next_instruction();
            }
            (0xE, _, 0x9, 0xE) => {
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed
                let x: usize = get_x(op_code) as usize;
                if self.keys[self.v[x] as usize] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            (0xE, _, 0xA, 0x1) => {
                // ExA1 - SKNP Vx
                // Skip next instruction if key with the value of Vx is not pressed
                let x: usize = get_x(op_code) as usize;
                if !self.keys[self.v[x] as usize] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            (0xF, _, 0x0, 0x7) => {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value
                let x: u16 = get_x(op_code);
                self.v[x as usize] = self.delay_timer;
                self.next_instruction();
            }
            (0xF, _, 0x0, 0xA) => {
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key in Vx.
                let x: usize = get_x(op_code) as usize;
                let mut is_pressed = false;
                for (i, key) in self.keys.iter().enumerate() {
                    if *key {
                        self.v[x] = i as u8;
                        is_pressed = true;
                        break;
                    }
                }

                if is_pressed {
                    self.next_instruction();
                }
            }
            (0xF, _, 0x1, 0x5) => {
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx
                let x: u16 = get_x(op_code);
                self.delay_timer = self.v[x as usize];
                self.next_instruction();
            }
            (0xF, _, 0x1, 0x8) => {
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                let x: u16 = get_x(op_code);
                self.sound_timer = self.v[x as usize];
                self.next_instruction();
            }
            (0xF, _, 0x1, 0xE) => {
                // Fx1E - ADD I, Vx
                // Set I = I + Vx
                let x: u16 = get_x(op_code);
                self.i = self.i + self.v[x as usize] as u16;
                self.next_instruction();
            }
            (0xF, _, 0x2, 0x9) => {
                // Fx29 - LD F, Vx
                // Set I = location of sprite for digit Vx.
                let x: usize = get_x(op_code) as usize;
                self.i = (self.v[x] * SPRITE_LENGTH) as u16;
                self.next_instruction();
            }
            (0xF, _, 0x3, 0x3) => {
                // Fx33 - LD B, Vx
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                let x: usize = get_x(op_code) as usize;
                let i: usize = self.i as usize;
                self.ram[i] = self.v[x] / 100;
                self.ram[i + 1] = (self.v[x] % 100) / 10;
                self.ram[i + 2] = self.v[x] % 10;
                self.next_instruction();
            }
            (0xF, _, 0x5, 0x5) => {
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at location I.
                let x: u16 = get_x(op_code);
                for index in 0..=x {
                    self.ram[(self.i + index) as usize] = self.v[index as usize];
                }

                // The original COSMAC VIP machine incremented I by one
                self.cosmac_increment_i();
                self.next_instruction();
            }
            (0xF, _, 0x6, 0x5) => {
                // Fx65 - LD Vx, [I]
                // Read registers V0 through Vx from memory starting at location I.
                let x: u16 = get_x(op_code);
                for register in 0..=x {
                    self.v[register as usize] = self.ram[(self.i + register) as usize];
                }

                // The original COSMAC VIP machine incremented I by one
                self.cosmac_increment_i();
                self.next_instruction();
            }
            _ => {
                println!("Instruction not supported by CHIP-8. Advancing to next instruction.");
                self.next_instruction();
            }
        }
    }

    /// Get an operation code using the program counter
    fn get_op_code(&self) -> u16 {
        // Cast is required because u8 can't be indexed by u16
        let addr: usize = self.pc as usize;
        (self.ram[addr] as u16) << 8 | (self.ram[addr + 1] as u16)
    }

    /// Increases the program counter by 2 to go to the next program instruction
    fn next_instruction(&mut self) {
        self.pc += 2;
    }

    /// Skip the next instruction in the program counter
    fn skip_instruction(&mut self) {
        self.pc += 4;
    }

    /// Jump to an instruction in the program counter
    fn jump_to_instruction(&mut self, instruction: u16) {
        self.pc = instruction;
    }

    /// Push to the CHIP-8 stack
    fn stack_push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }

    /// Pop from the CHIP-8 stack
    fn stack_pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        let value: u16 = self.stack[self.stack_pointer as usize];
        self.stack[self.stack_pointer as usize] = 0;
        value
    }

    /// Increments I by 1.
    /// The COSMAC VIP did this for the save and load opcodes Fx55 and Fx65 respectively.
    fn cosmac_increment_i(&mut self) {
        self.i += 1;
    }

    /// Clears the VF flag
    /// The COSMAC VIP did this for the bitwise operand opcodes 8xy1, 8xy2 and 8xy3.
    fn cosmac_clear_vf(&mut self) {
        self.v[0xF] = 0;
    }

    /// Sets the register VX to the value of VY
    /// The COSMAC VIP did this for the bitshift operand opcodes 8xy6 and 8xyE.
    fn cosmac_set_vx_to_vy(
        &mut self,
        x: usize,
        y: usize
    ) {
        self.v[x] = self.v[y];
    }
}
