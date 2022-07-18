pub mod audio;

pub struct Devices {
    pub audio: audio::AudioDevice,
}

impl Devices {
    fn new() -> Self {
        Devices {
            audio: audio::AudioDevice::default(),
        }
    }
}

pub fn configure_devices() -> std::io::Result<Devices> {
    Ok(Devices::new())
}
