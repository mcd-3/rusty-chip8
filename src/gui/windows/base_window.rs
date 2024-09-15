use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::Window;
use sdl2::EventPump;

pub struct SDLWindow {
    pub window: Window,
    pub event_pump: EventPump
}

impl SDLWindow {
    pub fn new(
        sdl: &Sdl,
        width: usize,
        height: usize,
        title: String,
    ) -> Result<Self, &'static str> {
        let video_subsystem: VideoSubsystem = sdl.video().unwrap();

        // Initiate the window
        let window: Window = video_subsystem
            .window(&title, width as u32, height as u32)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let event_pump: sdl2::EventPump =  sdl.event_pump().unwrap();

        Ok(SDLWindow {
            window,
            event_pump
        })
    }
}