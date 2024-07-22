use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{ BufReader, BufWriter, Result };
use crate::smtp::Connection;  // Assuming Connection is defined in the parent module
use crate::smtp::Message;
use crate::smtp_client::send_to_remote_smtp;
use crate::dns_lookup::*;

// fn handle_client(stream: TcpStream) {
//     let mut reader = BufReader::new(&stream);
//     let mut writer = BufWriter::new(&stream);
//     
//     match Connection::handle(&mut reader, &mut writer) {
//         Ok(_) => println!("Connection handled successfully"),
//         Err(e) => eprintln!("Error handling connection: {}", e),
//     }
// }
const LOCAL_DOMAIN: &str = "yourdomain.com";
fn extract_email(command: &str) -> &str { command.trim_start_matches('<').trim_end_matches('>') }

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    
    match Connection::handle(&mut reader, &mut writer) {
        Ok(connection) => {
            if let Some(messages) = connection.get_messages() {
                for message in messages { distribute_message(message); }
            }
            println!("Connection handled successfully")
        },
        Err(e) => eprintln!("Error handling connection: {}", e),
    }
}
fn distribute_message(message: &Message) {
    for recipient in message.get_recipients() {
        if is_local_recipient(recipient, LOCAL_DOMAIN) {
            println!("Delivered to local mailbox.")
        } else {
            match lookup_mx_record(extract_email(recipient)) {
                Ok(remote_server) => {
                    match send_to_remote_smtp(message, &remote_server) {
                        Ok(_) => println!("Sent to remote SMTP server for {}", recipient),
                        Err(e) => eprintln!("Failed to send to remote SMTP for {}: {}", recipient, e),
                    }
                },
                Err(e) => eprintln!("Failed to lookup MX record for {}: {}", recipient, e),
            }
        }
    }
}

pub fn start_server(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }

    Ok(())
}
