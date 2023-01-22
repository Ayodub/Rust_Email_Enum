use std::fs::File;
use std::io::{BufRead, BufReader};
use lettre::email::EmailBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SMTPTransport;
use lettre::transport::EmailTransport;

fn main() {
    let file_name = "email_list.txt";
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Set up SMTP server connection
    let creds = Credentials::new("username", "password");
    let smtp_server = "smtp.example.com";
    let mut mailer = SMTPTransport::simple_builder(smtp_server)
        .credentials(creds)
        .build();

    // Read the file line by line and send email to each address
    for line in reader.lines() {
        let email_address = line.unwrap();
        let email = EmailBuilder::new()
            .to(emailaddress)
            .from("sender@example.com")
            .subject("Test Email")
            .text("This is a test email.")
            .build()
            .unwrap();
        // Send the email
        match mailer.send(email) {
            Ok() => println!("Successfully sent to {}", email_address),
            Err(e) => {
                println!("Failed to send to {}", email_address);
                println!("{}", e);
            }
        }
    }

    // Close the connection
    mailer.close();
}