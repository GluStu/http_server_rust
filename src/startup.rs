use actix_web::dev::Server;
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use crate::routes::{subscribe, health_check};


pub async fn run(listner: TcpListener) -> Result<Server, std::io::Error>  {

    let server = HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // .route("/", web::get().to(health_check))
    })
    .listen(listner)?
    .run();

    Ok(server)
}