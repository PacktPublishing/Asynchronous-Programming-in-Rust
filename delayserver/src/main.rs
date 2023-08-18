use std::time::Duration;
use actix_web::{Responder, HttpResponse, get, HttpServer, App, web, rt::time::sleep};

#[get("/{delay}/{text}")]
async fn delay(path: web::Path<(u64, String)>) -> impl Responder {
    let (delay_ms, message) = path.into_inner();
    println!("Got message: {message}");
    sleep(Duration::from_millis(delay_ms)).await;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(delay)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}