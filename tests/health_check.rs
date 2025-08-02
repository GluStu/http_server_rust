use std::net::TcpListener;
use reqwest::Client;

#[tokio::test]
async fn  health_check_test() {
    let address: String = spawn_app().await;
    let client = Client::new();
    let response = client
                                    .get(&format!("{}/health_check", &address))
                                    .send()
                                    .await
                                    .expect("req exec fail");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("binding failed");
    let port = listner.local_addr().unwrap().port();
    let server = rust_mail::run(listner).await.expect("failed to bind");
    let _ = tokio::spawn(server);
    return format!("http://127.0.0.1:{}", port);
}