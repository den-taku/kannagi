use log::{debug, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    kannagi::utils::logging::configure_logger();
    debug!("logger configured.");

    kannagi::devices::configure_devices()?;

    info!("Server running!");
    let port = 8080;
    kannagi::server::run_server(port).await
}
