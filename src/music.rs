use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound, stop_sound};
pub struct Music {
    theme_music: Sound,
}

impl Music {
    pub async fn new() -> Music {
        let theme_music = load_sound("8bit-spaceshooter.ogg").await.unwrap();
        Music { theme_music }
    }

    pub fn reset(&mut self) {
        self.stop();
        self.play();
    }

    pub fn stop(&mut self) {
        stop_sound(&self.theme_music);
    }

    pub fn play(&mut self) {
        play_sound(
            &self.theme_music,
            PlaySoundParams {
                looped: true,
                volume: 1.,
            },
        );
    }
}
