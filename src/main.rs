use actix_web::{get, middleware, web, App, HttpServer, Responder};
use log::{debug, info, trace};

#[get("/")]
async fn root() -> impl Responder {
    "Root: Ok\n".to_string()
}

async fn test() -> String {
    "test: Ok\n".to_string()
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello!\n".to_string()
}

fn app_server(cfg: &mut web::ServiceConfig) {
    trace!("service configured");
    cfg.service(hello);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    kannagi::utils::logging::configure_logger();
    debug!("logger configured.");

    info!("Server running!");
    let host = "0.0.0.0";
    let port = 8080;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_server)
            .route("/test", web::get().to(test))
            .service(root)
    })
    .bind((host, port))?
    .run()
    .await;
    kannagi::server::server();
    server
}
