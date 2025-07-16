use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub fn send_auto_reply(to: &str, ticket_subject: &str, ticket_id: &str) -> Result<(), Box<dyn std::error::Error>>{
    let email_body = format!(
        "Hi,\n\nWe've received your Complaint:\n\nSubject: {}\n\nComplaint ID: {}\n\nOur support team will respond shortly.\n\nThank you.",
        ticket_subject, ticket_id
    );

    let email = Message::builder()
    .from("Support Team <support@test.com>".parse()?)
    .to(to.parse()?)
    .body(email_body)?;

    let creds = Credentials::new(
        "testmailx42@gmail.com".to_string(),
        "cgtq ozai caxy dpin".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    mailer.send(&email)?;

    println!("📤 Sent auto-reply to {}", to);
    Ok(())
}