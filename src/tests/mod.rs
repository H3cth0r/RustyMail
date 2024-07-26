#[cfg(test)]
mod tests {
    use crate::db::{MongoDB, DatabaseOperations};
    use tokio;
    use std::net::TcpStream;
    use std::io::{Write, BufReader, BufRead};
    use crate::models::user::User;
    use crate::models::email::Email;

    async fn setup_test_db() -> MongoDB {
        MongoDB::connect("mongodb://localhost:27017").await.expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_create_user() {
        let db = setup_test_db().await;
        let user = User {
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            aliases: vec![],
        };
        db.create_user(&user).await.expect("Failed to create user");

        let retrieved_user = db.get_user("test@example.com").await.expect("Failed to retrieve user");
        assert!(retrieved_user.is_some());
        assert_eq!(retrieved_user.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_store_and_retrieve_email() {
        let db = setup_test_db().await;
        let email = Email {
            from: "sender@example.com".to_string(),
            to: vec!["recipient@example.com".to_string()],
            subject: "Test Email".to_string(),
            body: "This is a test email".to_string(),
            timestamp: chrono::Utc::now(),
            folder: "inbox".to_string(),
        };
        
        match db.store_email(&email, "recipient@example.com", "inbox").await {
            Ok(_) => println!("Email stored successfully"),
            Err(e) => println!("Failed to store email: {}", e),
        }
    
        let retrieved_emails = db.get_emails("recipient@example.com", "inbox").await.expect("Failed to retrieve emails");
        println!("Number of retrieved emails: {}", retrieved_emails.len());
        
        for (i, email) in retrieved_emails.iter().enumerate() {
            println!("Email {}: From: {}, Subject: {}", i, email.from, email.subject);
        }
    
        assert_eq!(retrieved_emails.len(), 1, "Expected 1 email, but found {}", retrieved_emails.len());
        if !retrieved_emails.is_empty() {
            assert_eq!(retrieved_emails[0].subject, "Test Email", "Subject mismatch");
        }
    }

    // #[tokio::test]
    // async fn test_smtp_server() {
    //     // Start the SMTP server in a separate task
    //     tokio::spawn(async {
    //         crate::server::start_server("127.0.0.1:2525").await.expect("Failed to start SMTP server");
    //     });

    //     // Give the server a moment to start
    //     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    //     // Connect to the SMTP server
    //     let mut stream = TcpStream::connect("127.0.0.1:2525").expect("Failed to connect to SMTP server");
    //     let mut reader = BufReader::new(stream.try_clone().unwrap());

    //     // Read the welcome message
    //     let mut response = String::new();
    //     reader.read_line(&mut response).expect("Failed to read server response");
    //     assert!(response.starts_with("220"));

    //     // Send HELO command
    //     stream.write_all(b"HELO example.com\r\n").expect("Failed to send HELO command");
    //     let mut response = String::new();
    //     reader.read_line(&mut response).expect("Failed to read server response");
    //     assert!(response.starts_with("250"));

    //     // Send QUIT command
    //     stream.write_all(b"QUIT\r\n").expect("Failed to send QUIT command");
    //     let mut response = String::new();
    //     reader.read_line(&mut response).expect("Failed to read server response");
    //     assert!(response.starts_with("221"));
    // }
}

