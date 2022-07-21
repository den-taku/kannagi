use crate::server::controllers::*;
use actix_web::web;
use log::trace;

pub fn configure_server(cfg: &mut web::ServiceConfig) {
    trace!("service configured");
    cfg.route("/", web::get().to(root))
        .route("/test", web::get().to(test))
        .route("/hello", web::get().to(hello))
        .route("/sound/wav", web::get().to(sound_test_wav))
        .route("/sound/m4a", web::get().to(sound_test_m4a))
        .route("/sound/ser_volume", web::get().to(sound_set_volume))
        .route("/sound/ser_speed", web::get().to(sound_set_speed))
        .route("/sound/play", web::get().to(sound_play))
        .route("/sound/pause", web::get().to(sound_pause))
        .route("/sound/clear", web::get().to(sound_clear))
        .route("/sound/append", web::get().to(sound_append));
}
