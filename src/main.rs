use std::io::Result;
use rust_mail::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener).await?.await
}