use raylib::prelude::*;

pub struct Audio<'a> {
    audio_device: RaylibAudio,
    beep: Sound<'a>
}

impl <'a>Audio<'a> {
    pub fn new() -> Self {
        let audio_device = RaylibAudio::init_audio_device().expect("Failed to initialize sound driver.");
        let filename = "beep.wav";
        let beep: Sound<'_> = audio_device.new_sound(filename).expect("Failed to load beep sound.");

        Self { audio_device, beep}
    }

    pub fn beep(&self) {
        self.beep.play();
    }
}