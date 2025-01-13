use salvo::prelude::*;
use std::collections::HashMap;
use std::str;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use once_cell::sync::Lazy;
use config::Config;
use tracing::{info, warn, error};

#[derive(Debug, serde::Deserialize)]
struct AppConfig {
    smtp_relay: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
    to_email: String,
    bcc_email: String,
    webhook_path: String,
    host: String,
    email_subject: String,
    email_message_prefix: String,
    account_sid: String,
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
            error!("Failed to read body: {}", err);
            return;
        }
    };

    let body_str = match str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => {
            error!("Body contains invalid UTF-8 data");
            return;
        }
    };

    info!("Body: {}", body_str);

    let params: HashMap<_, _> =
        url::form_urlencoded::parse(body_str.as_bytes()).into_owned().collect();

    // Extract AccountSid from params and validate it
    if let Some(account_sid) = params.get("AccountSid") {
        if account_sid != &CONFIG.account_sid {
            error!(
                "AccountSid mismatch: received {}, expected {}",
                account_sid, CONFIG.account_sid
            );
            return;
        }
    } else {
        error!("Missing AccountSid in request");
        return;
    }

    if let (Some(to), Some(from), Some(body)) =
        (params.get("To"), params.get("From"), params.get("Body"))
    {
        info!("To: {}, From: {}, Body: {}", to, from, body);

        let email_content = format!(
            "{}\nTo: {}\nFrom: {}\nBody: {}",
            CONFIG.email_message_prefix, to, from, body
        );

        if let Err(err) = send_email(
            &CONFIG.to_email,
            &CONFIG.bcc_email,
            &CONFIG.email_subject,
            &email_content,
        )
        .await
        {
            error!("Failed to send email: {}", err);
        } else {
            info!("Email sent successfully!");
        }
    } else {
        warn!("Missing required fields: To, From, or Body");
    }
}

async fn send_email(
    to: &str,
    bcc: &str,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(CONFIG.from_email.parse()?)
        .to(to.parse()?)
        .bcc(bcc.parse()?) // Add BCC recipient
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
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Define a router with a POST handler for the webhook endpoint
    let router = Router::new()
        .path(&CONFIG.webhook_path)
        .post(handle_webhook);

    // Bind the router to a listener
    let acceptor = TcpListener::new(&CONFIG.host).bind().await;

    info!(
        "Server running at http://{}/{}",
        CONFIG.host,
        CONFIG.webhook_path.trim_start_matches('/')
    );

    // Start the server
    Server::new(acceptor).serve(router).await;
}

