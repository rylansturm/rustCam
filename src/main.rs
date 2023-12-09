#![warn(clippy::pedantic)]

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, http::header};
use rscam::{Camera, Config};
use std::{fs, io::Write};


#[get("/index.html")]
async fn index() -> impl Responder {
    "Hello, world!".to_string()
}

#[get("/image")]
async fn image() -> impl Responder {
    let mut camera = rscam::new("/dev/video11").expect("Bad Camera");
    camera.start(&Config{
        interval: (1, 30),
        resolution: (1280, 720),
        format: b"MJPG",
        ..Default::default()
    }).unwrap();
    let frame = camera.capture().unwrap();
    let image_data = frame.to_vec();

    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(image_data)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::redirect("/", "/index.html"))
            .service(index)
            .service(image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
