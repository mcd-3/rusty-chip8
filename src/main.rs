mod gui {
    pub mod windows {
        pub mod base_window;
    }
}

mod drivers {
    pub mod rom_driver;
    pub mod graphics_driver;
    pub mod keyboard_driver;
    pub mod sound_driver;
}

mod chip8 {
    pub mod cpu;
    pub mod font;
    pub mod op_code_variable_util;
}

use drivers::graphics_driver::draw_to_screen;
use drivers::keyboard_driver::keyboard_to_keypad;
use gui::windows::base_window::SDLWindow;
use drivers::rom_driver;
use drivers::sound_driver;
use sdl2::{Sdl};
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::render::Canvas;
use sdl2::video::Window;
use chip8::cpu::CHIP8;

use std::{thread, time};

fn main(){
    let sdl: Sdl = sdl2::init().unwrap();
    let main_window_title: String = String::from("CHIP-8 Emulator");
    let mut window: SDLWindow = SDLWindow::new(
        &sdl,
        725,
        375,
        main_window_title
    ).unwrap();

    // Create canvas
    let mut canvas : Canvas<Window> = window.window.into_canvas()
        // .present_vsync()
        .build()
        .unwrap();

    // Create soundcard
    let mut sound_system = sound_driver::create_sound_card(&sdl);

    // Load rom into memory
    let buffer: Vec<u8> = rom_driver::read_rom_data(String::from("roms/quirks_test.ch8"));
    let mut processor: CHIP8 = CHIP8::new();

    processor.load_rom_data(&buffer);

    // Start the SDL2 application
    'gameloop: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                Event::KeyDown { keycode: Some(key ), ..} => {
                    if let Some(k) = keyboard_to_keypad(key) {
                        processor.press_key(k as usize, true);
                    }
                    sound_driver::play_sound(&sound_system);
                }
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = keyboard_to_keypad(key) {
                        processor.press_key(k as usize, false);
                    }
                    sound_driver::stop_sound(&sound_system);
                },
                _ => {   }
            }
        }


        processor.tick();
        draw_to_screen(processor.vram, &mut canvas);
        // let ten_millis = time::Duration::from_millis(1000 / 240);
        // let now = time::Instant::now();
        // thread::sleep(ten_millis);
    }
}
