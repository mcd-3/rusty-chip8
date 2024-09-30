use sdl2::{AudioSubsystem, Sdl};
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

pub struct SoundDriver {
    audio: AudioDevice<SquareWave>
}

impl SoundDriver {
    pub fn new(sdl: &Sdl) -> Result<Self, &'static str> {
        let audio: AudioSubsystem = sdl.audio().unwrap();
    
        let spec: AudioSpecDesired = AudioSpecDesired {
            freq: Some(44100), //44.1 kHz audio
            channels: Some(1), // mono channel
            samples: None,
        };
    
        let sc: AudioDevice<SquareWave> = audio.open_playback(None, &spec, |audio_spec| {
            SquareWave {
                phase_inc: 240.0 / audio_spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }).unwrap();

        Ok(SoundDriver {
            audio: sc
        })
    }
    
    /// Start the audio playback
    pub fn play_sound(&self) {
        self.audio.resume();
    }

    /// Stop the audio playback
    pub fn stop_sound(&self) {
        self.audio.pause();
    }
}

struct SquareWave {
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