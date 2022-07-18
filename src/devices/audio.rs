use crossbeam::channel::unbounded;
use log::info;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub enum AudioSignal {
    SetVolume(f32),
    SetSpeed(f32),
    Play,
    Pause,
    Stop,
    Append(usize),
}

pub struct AudioDevice {
    pub tx: crossbeam::channel::Sender<AudioSignal>,
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioDevice {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        std::thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            loop {
                use AudioSignal::*;
                match rx.recv().unwrap() {
                    SetVolume(value) => {
                        info!("SetVolume({value}) recieved");
                        sink.set_volume(value);
                    }
                    SetSpeed(value) => {
                        info!("SetSpeed({value}) recieved");
                        sink.set_speed(value);
                    }
                    Play => {
                        info!("Play recieved");
                        sink.play()
                    }
                    Pause => {
                        info!("Pause recieved");
                        sink.pause();
                    }
                    Stop => {
                        info!("Stop recieved");
                        sink.stop();
                    }
                    Append(id) => {
                        info!("Append({id}) recieved");
                        // sink.stop();
                        let file = BufReader::new(File::open("./samples/sample.wav").unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                    }
                }
            }
        });
        Self { tx }
    }
}

pub fn play_test_wav() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("./samples/sample.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

pub fn play_test_m4a() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open("./samples/sample2.m4a").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
