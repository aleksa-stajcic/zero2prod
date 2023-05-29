#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use actix_web::{Responder, HttpServer, App, web, HttpResponse};

async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    return HttpServer::new(|| {
        return App::new()
            .route("/health_check", web::get().to(health_check))
        ;
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await;
}
