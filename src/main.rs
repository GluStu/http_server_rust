use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io::Result;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("you");
    return format!("Hey {}", &name);
}

async fn health_check() -> impl Responder{
    HttpResponse::Ok()
    }

#[tokio::main]
async fn main() ->Result<()> {
    
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}