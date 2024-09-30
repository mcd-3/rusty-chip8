#![windows_subsystem = "windows"]

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
    pub mod op_code;
}

pub mod debug {
    pub mod debugger;
}

use chip8::cpu::CHIP8;
use drivers::graphics_driver::GraphicsDriver;
use drivers::keyboard_driver::KeyboardDriver;
use drivers::rom_driver::RomDriver;
use drivers::sound_driver::SoundDriver;
use gui::windows::base_window::SDLWindow;
use native_dialog::FileDialog;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::path::PathBuf;

fn main(){
    // We need to declare SDL first before the file dialog
    // This way we can capture keystrokes
    let sdl: Sdl = sdl2::init().unwrap();
    let main_window_title: String = String::from("CHIP-8 Emulator");
    let mut window: SDLWindow = SDLWindow::new(
        &sdl,
        725,
        375,
        main_window_title
    ).unwrap();

    let path: Option<PathBuf> = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("CHIP-8 ROM", &["ch8"])
        .show_open_single_file()
        .unwrap();

    let path: PathBuf = match path {
        Some(path) => path,
        None => return,
    };

    // Create canvas
    let canvas : Canvas<Window> = window.window.into_canvas()
        // .present_vsync()
        .build()
        .unwrap();

    // Load rom into memory
    let rom_driver: RomDriver = RomDriver::new(path).unwrap();
    let buffer: Vec<u8> = match rom_driver.read_rom_data() {
        Ok(path) => path,
        Err(e) => panic!("{}", e)
    };

    let mut processor: CHIP8 = CHIP8::new();

    // Create Drivers
    let keyboard_driver: KeyboardDriver = KeyboardDriver::new().unwrap();
    let mut graphics_driver: GraphicsDriver = GraphicsDriver::new(processor.vram, canvas).unwrap();
    let sound_driver: SoundDriver = SoundDriver::new(&sdl).unwrap();

    processor.load_rom_data(&buffer);

    // Start the SDL2 application
    'gameloop: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                Event::KeyDown { keycode: Some(key ), ..} => {
                    if let Some(k) = keyboard_driver.keyboard_to_keypad(key) {
                        processor.press_key(k as usize, true);
                    }
                }
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = keyboard_driver.keyboard_to_keypad(key) {
                        processor.press_key(k as usize, false);
                    }
                },
                _ => {   }
            }
        }

        if processor.sound_timer > 0 {
            sound_driver.play_sound();
        } else {
            sound_driver.stop_sound();
        }

        processor.tick();
        graphics_driver.update_vram(processor.vram);
        graphics_driver.draw_to_screen();
    }
}
