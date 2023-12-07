use actix_web::{get, web, App, HttpServer, Responder};

#[get("/index.html")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::redirect("/", "/index.html"))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}