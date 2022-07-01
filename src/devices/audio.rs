use ears::{AudioController, Sound};

pub fn play_test() {
    let mut sound = Sound::new("../samples/sample.wav").unwrap();
    loop {
        sound.play();
        while sound.is_playing() {}
    }
}
