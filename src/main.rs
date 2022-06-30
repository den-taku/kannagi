use actix_web::{get, middleware, web, App, HttpServer, Responder};
use file_rotate::{
    compression::Compression, suffix::AppendTimestamp, suffix::FileLimit, ContentLimit, FileRotate,
};
use log::{debug, info, trace};
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

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
    // Configure logger
    CombinedLogger::init(vec![
        // stdio
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // file
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            FileRotate::new(
                "target/trace_log",
                AppendTimestamp::default(FileLimit::MaxFiles(10)),
                ContentLimit::Lines(100_000),
                Compression::None,
            ),
        ),
    ])
    .unwrap();
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
    server
}
