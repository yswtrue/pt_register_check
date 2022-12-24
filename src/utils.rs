use anyhow::Result;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
pub async fn send_email(title: String, body: String) -> Result<()> {
    log::debug!("{}", format!("Sending email to {}", title));
    let email = Message::builder()
        .from(std::env::var("SEND_EMAIL")?.as_str().parse().unwrap())
        .to(std::env::var("RECEIVE_EMAIL")?.as_str().parse().unwrap())
        .subject(title)
        .body(body)
        .unwrap();

    let creds = Credentials::new(std::env::var("SMTP_USER")?, std::env::var("SMTP_PASSWORD")?);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(std::env::var("SMTP_HOST")?.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    return match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}
