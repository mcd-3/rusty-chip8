use sdl2::event::Event;

mod gui {
    pub mod windows {
        pub mod base_window;
    }
}

use gui::windows::base_window::SDLWindow;

fn main(){
    let main_window_title = String::from("CHIP-8 Emulator");
    let mut window = SDLWindow::new(800, 600, main_window_title).unwrap();

    // Test code to read rom data
    match std::fs::read("roms/test.ch8") {
        Ok(bytes) => {
            for (i, byte) in bytes.iter().enumerate() {
                println!("{:#04X}", byte);
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
                return;
            }
            panic!("{}", e);
        }
    }

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
