use crate::devices::audio::AudioSignal::{self, *};
use actix_web::{web, Responder};
use crossbeam::channel::Sender;
use std::sync::Mutex;

pub async fn sound_set_volume(_data: web::Data<Mutex<Sender<AudioSignal>>>) -> impl Responder {
    unimplemented!();
    #[allow(unreachable_code)]
    "Ok\n".to_string()
}

pub async fn sound_set_speed(_data: web::Data<Mutex<Sender<AudioSignal>>>) -> impl Responder {
    unimplemented!();
    #[allow(unreachable_code)]
    "Ok\n".to_string()
}

pub async fn sound_play(data: web::Data<Mutex<Sender<AudioSignal>>>) -> impl Responder {
    let tx = data.lock().unwrap();
    tx.send(Play).unwrap();
    "Ok\n".to_string()
}

pub async fn sound_pause(data: web::Data<Mutex<Sender<AudioSignal>>>) -> impl Responder {
    let tx = data.lock().unwrap();
    tx.send(Pause).unwrap();
    "Ok\n".to_string()
}

pub async fn sound_stop(data: web::Data<Mutex<Sender<AudioSignal>>>) -> impl Responder {
    let tx = data.lock().unwrap();
    tx.send(Stop).unwrap();
    "Ok\n".to_string()
}

pub async fn sound_append(data: web::Data<Mutex<Sender<AudioSignal>>>) -> impl Responder {
    let tx = data.lock().unwrap();
    tx.send(Append(0)).unwrap();
    "Ok\n".to_string()
}

pub async fn sound_test_wav() -> impl Responder {
    crate::devices::audio::play_test_wav();
    "Ok\n".to_string()
}

pub async fn sound_test_m4a() -> impl Responder {
    crate::devices::audio::play_test_m4a();
    "Ok\n".to_string()
}

pub async fn root() -> impl Responder {
    "Root: Ok\n".to_string()
}

pub async fn test() -> String {
    "test: Ok\n".to_string()
}

pub async fn hello() -> impl Responder {
    "Hello!\n".to_string()
}
