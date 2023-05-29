#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use actix_web::{HttpRequest, Responder, HttpServer, App, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    return format!("Hello {}!", &name);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    return HttpServer::new(|| {
        return App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet));
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await;
}
