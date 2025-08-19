use std::io::Result;
use rust_mail::startup::run;
use std::net::TcpListener;
use rust_mail::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<()>{
    let configuration = get_configuration().expect("config fail");
    let address = format!("127.0.0.{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("binding to port 8000 failed");
    run(listener).await.expect("failed to bind").await
}