use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use std::sync::atomic::{AtomicUsize, Ordering};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncStd1Executor, Message, AsyncTransport,
};

// Using atomic counter for thread-safe counting
static COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn print_email(email: String) -> HttpResponse {
    // Spawn a new tokio task to handle the printing
    tokio::spawn(async move {
        let otp_details = email.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut otp_details = otp_details.iter();
        let email = otp_details.next().unwrap().to_owned().to_owned();
        let otp = otp_details.next().unwrap().to_owned().to_owned();
        let reason = otp_details.next().unwrap().to_owned().to_owned();
        let username = std::env::var("USERNAME").expect("missing USERNAME in env");
        let token = std::env::var("TOKEN").expect("missing TOKEN in env");
        let server = std::env::var("SERVER").expect("missing SERVER in env");
        let display_name = std::env::var("DISPLAY_NAME").expect("missing DISPLAY_NAME in env");
        println!("email: {}\nOTP: {}\nreason: {}", email, otp, reason);
        let email = Message::builder()
            .from(format!("{} <{}>",display_name, username).parse().unwrap())
            .to(format!("user <{}>", email).parse().unwrap())
            .subject("Email verification / other service")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Test 1 2 3\n\nHI!"))
            .unwrap();

        let creds = Credentials::new(username.to_string(), token.to_string());

        // Open a remote connection to gmail using STARTTLS
        let mailer: AsyncSmtpTransport<AsyncStd1Executor> =
            AsyncSmtpTransport::<AsyncStd1Executor>::starttls_relay(&server.to_string())
                .unwrap()
                .credentials(creds)
                .build();

        // Send the email
        match mailer.send(email).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
        // just need to send the actual email now using creds from env and then go back to dashboard for verification
    });

    HttpResponse::Ok().body("Message processing started")
}

async fn root_handler() -> HttpResponse {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok().body("Hello, you've reached the root path!")
}

async fn stats_handler() -> HttpResponse {
    let count = COUNTER.load(Ordering::SeqCst);
    HttpResponse::Ok().body(count.to_string())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on :8080...");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t %r %a")) // Format: timestamp, request, remote addr
            .route("/", web::get().to(root_handler))
            .route("/stats", web::get().to(stats_handler))
            .route("/mail", web::post().to(print_email))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
