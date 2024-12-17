use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use std::sync::atomic::{AtomicUsize, Ordering};

// Using atomic counter for thread-safe counting
static COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn root_handler() -> HttpResponse {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok().body("Hello, you've reached the root path!")
}

async fn stats_handler() -> HttpResponse {
    let count = COUNTER.load(Ordering::SeqCst);
    HttpResponse::Ok().body(count.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Server starting on :8080...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t %r %a")) // Format: timestamp, request, remote addr
            .route("/", web::get().to(root_handler))
            .route("/stats", web::get().to(stats_handler))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
