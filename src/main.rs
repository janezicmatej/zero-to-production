use std::net::TcpListener;

use ztp::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    run(listener)?.await
}
