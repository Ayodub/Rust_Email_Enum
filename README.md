# Rust_Email_Enum
Iterate through a list to identfy valid emails

Reads a file named email_list.txt line by line, and for each line, it sends an email to the email address. It uses the SMTPTransport provided by the lettre library which is a simple way to send email via SMTP. It sets up the SMTP server connection using the credentials provided and sends the email. Based on the response it returns the status of the email sent. It also closes the connection after sending all the emails.

Please note that this example assumes that the file email_list.txt is in the same directory as the program and contains one email address per line. Also, you will have to replace the SMTP server address, username, and password accordingly
