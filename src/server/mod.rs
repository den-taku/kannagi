use actix_web::{middleware, App, HttpServer};

mod controller;
mod service;

pub async fn run_server(port: u16) -> std::io::Result<()> {
    let host = "0.0.0.0";
    let workers = 4;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(service::configure_server)
    })
    .bind((host, port))?
    .workers(workers)
    .run()
    .await
}
