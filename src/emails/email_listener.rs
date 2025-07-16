use imap::Session;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::time::Duration;
use sqlx::PgPool;
use uuid::Uuid;
use mailparse::{parse_mail, MailHeaderMap};
use imap::types::Fetch;
use tokio::time::sleep;

use crate::emails::smtp::send_auto_reply;

pub async fn start_email_watcher(pool: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(e) = check_inbox_and_create_tickets(&pool).await {
                eprintln!("❌ Email check failed: {}", e);
            }
            sleep(Duration::from_secs(30)).await;
        }
    });
}

async fn check_inbox_and_create_tickets(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let domain = "imap.gmail.com";
    let email = "testmailx42@gmail.com";
    let password = "cgtq ozai caxy dpin";

    // Connect to IMAP server
    let tls = TlsConnector::builder().build()?;
    let client = imap::connect((domain, 993), domain, &tls)?;
    let mut session = client.login(email, password).map_err(|e| e.0)?;

    // Select inbox
    session.select("INBOX")?;

    // Search for unseen messages
    let unseen = session.search("UNSEEN")?;

    for msg_id in unseen.iter() {
        let messages = session.fetch(msg_id.to_string(), "RFC822")?;
        for message in messages.iter() {
            if let Some(body) = message.body() {
                let parsed = parse_mail(body)?;

                let subject = parsed.headers.get_first_value("Subject").unwrap_or("No subject".to_string());
                let from = parsed.headers.get_first_value("From").unwrap_or("unknown@example.com".to_string());
                let body_text = parsed.get_body().unwrap_or("No body".to_string());

                println!("📩 New email:\nFrom: {}\nSubject: {}\n---\n{}", from, subject, body_text);

                insert_ticket_from_email(pool, &subject, &body_text, &from).await?;
            }
        }

        // Mark as seen
        session.store(msg_id.to_string(), "+FLAGS (\\Seen)")?;
    }

    session.logout()?;
    Ok(())
}

async fn insert_ticket_from_email(
    pool: &PgPool,
    subject: &str,
    body: &str,
    from: &str,
) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO tickets (id, title, description, status, priority, customer_email)
        VALUES ($1, $2, $3, 'Open', 'Medium', $4)
        "#,
        id,
        subject,
        body,
        from
    )
    .execute(pool)
    .await?;

    println!("✅ Ticket inserted for email: {}", subject);

    send_auto_reply(from, subject, &id.to_string());

    Ok(())
}
