use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

// TODO: placeholder 

pub fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("your@example.com".parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_owned())?;
    let creds = Credentials::new("smtp_username".into(), "smtp_password".into());

    let mailer = SmtpTransport::relay("smtp.example.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    Ok(())
}