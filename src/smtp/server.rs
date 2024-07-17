use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::smtp::commands::handle_command;

pub async fn run_smtp_server() {
    let listener = TcpListener::bind("127.0.0.1:2525").await.unwrap();
    println!("SMTP server running on 127.0.0.1:2525");
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let mut data_mode = false;
            let mut email_data = String::new();
            socket.write_all(b"220 RustyMail Server ready\r\n").await.unwrap();
            
            loop {
                let n = socket.read(&mut buffer).await.unwrap();
                if n == 0 {
                    break;
                }
                let input = String::from_utf8_lossy(&buffer[..n]);
                let lines: Vec<&str> = input.split("\r\n").collect();
                
                for line in lines {
                    if line.is_empty() {
                        continue;
                    }
                    if data_mode {
                        if line.trim() == "." {
                            data_mode = false;
                            println!("Email data received:\n{}", email_data);
                            email_data.clear();
                            let response = "250 OK\r\n".to_string();
                            println!("Responding with: {}", response.trim_end());
                            socket.write_all(response.as_bytes()).await.unwrap();
                        } else {
                            email_data.push_str(line);
                            email_data.push_str("\r\n");
                        }
                    } else {
                        let response = handle_command(line);
                        if line.trim().eq_ignore_ascii_case("DATA") {
                            data_mode = true;
                        }
                        println!("Responding with: {}", response.trim_end());
                        socket.write_all(response.as_bytes()).await.unwrap();
                        if line.trim().eq_ignore_ascii_case("QUIT") {
                            return;
                        }
                    }
                }
            }
        });
    }
}
