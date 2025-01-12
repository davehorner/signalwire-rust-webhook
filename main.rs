use salvo::prelude::*;
use std::collections::HashMap;
use std::str;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use once_cell::sync::Lazy;
use config::Config;

#[derive(Debug, Deserialize)]
struct AppConfig {
    smtp_relay: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
    to_email: String,
    webhook_path: String,
    host: String,
}

// Global configuration loaded once
static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Failed to load configuration")
        .try_deserialize::<AppConfig>()
        .expect("Failed to parse configuration")
});

#[handler]
async fn handle_webhook(req: &mut Request) {
    let body = match req.payload().await {
        Ok(bytes) => bytes,
        Err(err) => {
            println!("Failed to read body: {}", err);
            return;
        }
    };

    let body_str = match str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => {
            println!("Body contains invalid UTF-8 data");
            return;
        }
    };

    println!("Body: {}", body_str);

    let params: HashMap<_, _> =
        url::form_urlencoded::parse(body_str.as_bytes()).into_owned().collect();

    if let (Some(to), Some(from), Some(body)) =
        (params.get("To"), params.get("From"), params.get("Body"))
    {
        println!("To: {}, From: {}, Body: {}", to, from, body);

        let email_content = format!(
            "New message received:\n\nTo: {}\nFrom: {}\nBody: {}\n",
            to,
            from,
            body,
        );

        if let Err(err) = send_email(&CONFIG.to_email, "New Webhook Message", &email_content).await {
            println!("Failed to send email: {}", err);
        } else {
            println!("Email sent successfully!");
        }
    } else {
        println!("Missing required fields: To, From, or Body");
    }
}

async fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(CONFIG.from_email.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    let creds = Credentials::new(
        CONFIG.smtp_username.clone(),
        CONFIG.smtp_password.clone(),
    );

    let mailer = SmtpTransport::relay(&CONFIG.smtp_relay)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Define a router with a POST handler for the webhook endpoint
    let router = Router::new()
        .path(&CONFIG.webhook_path)
        .post(handle_webhook);

    // Bind the router to a listener
    let acceptor = TcpListener::new(&CONFIG.host).bind().await;

    println!(
        "Server running at http://{}/{}",
        CONFIG.host,
        CONFIG.webhook_path.trim_start_matches('/')
    );

    // Start the server
    Server::new(acceptor).serve(router).await;
}