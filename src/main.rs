mod gui {
    pub mod windows {
        pub mod base_window;
    }
}

mod drivers {
    pub mod rom_driver;
}

mod chip8 {
    pub mod cpu;
    pub mod font;
    pub mod op_code_variable_util;
}

use gui::windows::base_window::SDLWindow;
use drivers::rom_driver;
use sdl2::event::Event;
use chip8::cpu::CHIP8;

fn main(){
    let main_window_title: String = String::from("CHIP-8 Emulator");
    let mut window: SDLWindow = SDLWindow::new(800, 600, main_window_title).unwrap();

    // Load rom into memory
    let buffer: Vec<u8> = rom_driver::read_rom_data(String::from("roms/test.ch8"));
    for (i, byte) in buffer.iter().enumerate() {
        println!("{:#04X}", byte);
    }

    let mut processor: CHIP8 = CHIP8::new();

    processor.load_rom_data(&buffer);
    // processor.decrement_sound_timer();
    processor.run_next_instruction();
    processor.run_next_instruction();
    processor.run_next_instruction();
    processor.run_next_instruction();
    processor.run_next_instruction();
    processor.run_next_instruction();
    processor.run_next_instruction();
    processor.run_next_instruction();

    // Start the SDL2 application
    'running: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {   }
            }
        }
    }
}
