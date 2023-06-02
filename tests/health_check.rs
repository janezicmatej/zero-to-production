use std::net::TcpListener;

use ztp::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.1:0").expect("failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("failed to bind address");
    tokio::spawn(server);

    format!("http://127.1:{port}")
}

#[tokio::test]
async fn test_health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
