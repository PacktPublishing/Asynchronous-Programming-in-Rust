use std::{time::Duration, sync::atomic::{AtomicUsize, Ordering}};
use actix_web::{Responder, get, HttpServer, App, web, rt::time::sleep};

const EXPLANATION: &str =
"USAGE:
Delay server works by issuing a http GET request in the format:
http://localhost:8080/[delay in ms]/[UrlEncoded meesage]

On reception, it immidiately reports the following to the console:
{Message #} - {delay in ms}: {message}

The server then delays the response for the requested time and echoes the message back to the caller.

REQUESTS:
--------
";
static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[get("/{delay}/{message}")]
async fn delay(path: web::Path<(u64, String)>) -> impl Responder {
    let (delay_ms, message) = path.into_inner();
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("#{count} - {delay_ms}ms: {message}");
    sleep(Duration::from_millis(delay_ms)).await;
    message
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{EXPLANATION}");
    HttpServer::new(|| {
        App::new()
        .service(delay)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}