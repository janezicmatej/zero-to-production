use std::net::TcpListener;

use sqlx::{Connection, PgConnection};

use ztp::configuration::get_configuration;
use ztp::startup::run;

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

#[tokio::test]
async fn subscribe_valid_cases() {
    let address = spawn_app();
    let config = get_configuration().expect("failed reading configuration");
    let connection_string = config.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("failed to connect to pg");
    let client = reqwest::Client::new();

    let body = "name=le%20doge&email=muchwow%40gmail.com";
    let response = client
        .post(&format!("{address}/subscriptions"))
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");
}

#[tokio::test]
async fn subscribe_error_cases() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let tests = [
        ("name=ledoge", "missing email"),
        ("email=ledoge@gmail.com", "missing name"),
        ("", "missing email and name"),
    ];

    for (body, reason) in tests {
        let response = client
            .post(format!("{address}/subscriptions"))
            .body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "didn't fail with 400 when there was {reason}",
        );
    }
}
