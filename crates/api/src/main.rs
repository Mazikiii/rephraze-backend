use api::run;
use bootstrap::configuration::get_configurations;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configurations().expect("failded to load configuration file");

    let listener = TcpListener::bind("127.0.0.1:{}, config.application.port")?;

    run(listener)?.await
}
