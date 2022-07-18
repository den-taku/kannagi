use actix_web::{middleware, web, App, HttpServer};
use std::sync::Mutex;

mod controllers;
mod service;

pub async fn run_server(port: u16, devices: crate::devices::Devices) -> std::io::Result<()> {
    let host = "0.0.0.0";
    let workers = 4;

    let audio_channel = web::Data::new(Mutex::new(devices.audio.tx));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(audio_channel.clone())
            .configure(service::configure_server)
    })
    .bind((host, port))?
    .workers(workers)
    .run()
    .await
}
