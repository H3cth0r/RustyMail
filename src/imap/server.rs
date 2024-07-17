use tokio::net::TcpListener;
use tokio::prelude::*;
use create::imap::commands::handle_command;
use create::imap::responses::IMAPResponse;

pub async fn run_imap_server() {
    let listener = TcpListener::bind("127.0.0.1:143").await.unwrap();
    println!("IMAP server running on 127.0.0.1:143");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            socket.read(&mut buffer).await.unwrap();

            let command = String::from_utf8_lossy(&buffer[..]);
            let response = handle_command(&command);
            socket.write_all(response.to_string().as_bytes()).await.unwrap();
        });
    }
}
