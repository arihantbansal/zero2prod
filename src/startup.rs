use crate::routes;
use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::health_check)
            .service(routes::subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
