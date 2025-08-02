#[tokio::test]
async fn  health_check_test() {
    let _ = spawn_app();
}

async fn spawn_app(){
    let server = rust_mail::run().expect("failed to bind");
    let _ = tokio::spawn(server);
}