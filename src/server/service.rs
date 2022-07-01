use crate::server::controllers::*;
use actix_web::web;
use log::trace;

pub fn configure_server(cfg: &mut web::ServiceConfig) {
    trace!("service configured");
    cfg.route("/", web::get().to(root))
        .route("/test", web::get().to(test))
        .route("/hello", web::get().to(hello))
        .route("/sound", web::get().to(sound_test));
}
