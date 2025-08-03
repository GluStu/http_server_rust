use std::{net::TcpListener};
use reqwest::Client;

#[tokio::test]
async fn  health_check_test() {
    let address: String = spawn_app().await;
    let client = Client::new();
    let response: reqwest::Response = client
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
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_addr = spawn_app().await;
    let client = Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response: reqwest::Response = client
                                      .post(&format!("{}/subscriptions", &app_addr))
                                      .body(body)
                                      .send()
                                      .await
                                      .expect("Failed to execute req");
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_addr = spawn_app().await;
    let client = Client::new();
    let test_cases: Vec<(&'static str, &'static str)> = vec![("name=le%20guin", "missing the email"),
                                                            ("email=ursula_le_guin%40gmail.com", "missing the name"),
                                                            ("", "missing both name and email")];

    for (body, msg) in test_cases{
        let resp: reqwest::Response = client
                                      .post(format!("{}/subscriptions", &app_addr))
                                      .header("Content-Type", "application/x-www-form-urlencoded")
                                      .body(body)
                                      .send()
                                      .await
                                      .expect("Failed to execute req");
        assert_eq!(400, resp.status().as_u16(), "The API did not fail with 400 Bad Request when the payload was {msg}.");
    }
}