use actix_web::{post, App, HttpServer, Responder};

#[post("/")]
async fn http_parrot(data: String) -> impl Responder {
    data
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(http_parrot))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
