use actix_web::{get, rt::time::sleep, web, App, HttpServer, Responder};
use std::{
    env,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

const EXPLANATION: &str =
"USAGE:
Delay server works by issuing an HTTP GET request in the format:
http://localhost:8080/[delay in ms]/[URL-encoded message]

If an argument is passed in when delayserver is started, that
argument will be used as the URL instead of 'localhost'.

Upon receiving a request, it immediately reports the following to the console:

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
    let url = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost"));

    println!("{EXPLANATION}");
    HttpServer::new(|| App::new().service(delay))
        .bind((url, 8080))?
        .run()
        .await
}
