use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader, BufWriter, Result};
use crate::smtp::Connection;  // Assuming Connection is defined in the parent module

fn handle_client(stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    
    match Connection::handle(&mut reader, &mut writer) {
        Ok(_) => println!("Connection handled successfully"),
        Err(e) => eprintln!("Error handling connection: {}", e),
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
