use std::sync::Arc;
use std::net::TcpListener;
use rustls::{ServerConfig, ServerConnection};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // Load server certificate and private key
    println!("Current directory: {:?}", std::env::current_dir()?);
    let cert_file = &mut BufReader::new(File::open("server.crt")?);
    let key_file = &mut BufReader::new(File::open("server.key")?);
    
    let cert_chain = certs(cert_file)?.into_iter().map(rustls::Certificate).collect();
    let mut keys = pkcs8_private_keys(key_file)?;
    let private_key = rustls::PrivateKey(keys.remove(0));

    // Configure the server
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .expect("Failed to create server config");
    let config = Arc::new(config);

    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:8443")?;
    println!("Server listening on port 8443");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut connection = ServerConnection::new(Arc::clone(&config))
            .expect("Failed to create server connection");

        let mut tls_stream = rustls::Stream::new(&mut connection, &mut stream);

        // Handle the connection
        let mut buf = [0; 1024];
        let n = tls_stream.read(&mut buf)?;
        println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

        tls_stream.write_all(b"Hello from server!")?;
    }

    Ok(())
}
