use sdl2::Sdl;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

pub fn create_sound_card(sdl: &Sdl) -> AudioDevice<SquareWave> {
    let audio = sdl.audio().unwrap();

    let spec = AudioSpecDesired {
        freq: Some(44100), //44.1 kHz audio
        channels: Some(1), // mono channel
        samples: None,
    };

    let sc = audio.open_playback(None, &spec, |audio_spec| {
        // Show obtained AudioSpec
        println!("{:?}", audio_spec);

        SquareWave {
            phase_inc: 240.0 / audio_spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        }
    }).unwrap();

    sc
}

pub fn play_sound(sound_card: &AudioDevice<SquareWave>) {
    sound_card.resume();
}

pub fn stop_sound(sound_card: &AudioDevice<SquareWave>) {
    sound_card.pause();
}

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = self.volume * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}