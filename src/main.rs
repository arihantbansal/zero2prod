use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
