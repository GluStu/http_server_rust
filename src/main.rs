use std::io::Result;
use rust_mail::run;

#[tokio::main]
async fn main() -> Result<()>{
    run()?.await
}