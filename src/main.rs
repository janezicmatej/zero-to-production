use std::net::TcpListener;

// use ztp::configuration::get_configuration;
use ztp::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // this runs inside docker and should always be exposed on 0.0.0.0:8000
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    run(listener)?.await
}
