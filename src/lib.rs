use actix_web::{HttpServer, App, web, Responder, HttpResponse};


async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

pub async fn run() -> Result<(), std::io::Error> {
    return HttpServer::new(|| {
        return App::new()
            .route("/health_check", web::get().to(health_check))
        ;
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await;
}

