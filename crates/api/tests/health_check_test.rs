use std::net::TcpListener;

#[tokio::test]
pub async fn test_health_check_is_active() {
    spawn_app();

    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}api/v1/health-check", &address))
        .send()
        .await
        .expect("Failed to bind to server");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to get a port");
    let port = listener.local_addr().unwrap().port();

    let server = api::run(listener).expect("Could not bind server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
