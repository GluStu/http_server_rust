use std::io::Result;
use rust_mail::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8000").expect("binding to port 8000 failed");
    run(listener).await.expect("failed to bind").await
}