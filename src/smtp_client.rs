use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2525").await.unwrap();
    let mut buffer = [0; 1024];
    
    // Read the initial greeting
    stream.read(&mut buffer).await.unwrap();
    println!("Server: {}", String::from_utf8_lossy(&buffer[..]));

    async fn send_and_receive(stream: &mut TcpStream, command: &str) {
        println!("Client: {}", command.trim());
        stream.write_all(command.as_bytes()).await.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        println!("Server: {}", String::from_utf8_lossy(&buffer[..]));
    }

    send_and_receive(&mut stream, "HELO example.com\r\n").await;
    send_and_receive(&mut stream, "MAIL FROM:<sender@example.com>\r\n").await;
    send_and_receive(&mut stream, "RCPT TO:<recipient@example.com>\r\n").await;
    send_and_receive(&mut stream, "DATA\r\n").await;

    // Send email data
    println!("Client: Sending email data");
    stream.write_all(b"Subject: Test Email\r\n\r\nThis is a test email.\r\n.\r\n").await.unwrap();
    
    // Wait for the final response
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    println!("Server: {}", String::from_utf8_lossy(&buffer[..]));

    send_and_receive(&mut stream, "QUIT\r\n").await;
}
