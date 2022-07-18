use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

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
