use std::sync::Arc;
use std::net::TcpStream;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use rustls_pemfile::certs;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // Load root certificate
    let mut root_store = RootCertStore::empty();
    let cert_file = &mut BufReader::new(File::open("rootCA.crt")?);
    let certs = certs(cert_file)?;
    root_store.add_parsable_certificates(&certs);

    // Configure the client
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let config = Arc::new(config);

    // Connect to the server
    let server_name = "localhost".try_into().unwrap();
    let mut conn = ClientConnection::new(Arc::clone(&config), server_name)
        .expect("Failed to create client connection");
    let mut stream = TcpStream::connect("127.0.0.1:8443")?;
    let mut tls_stream = rustls::Stream::new(&mut conn, &mut stream);

    // Send a message
    tls_stream.write_all(b"Hello from client!")?;

    // Receive the response
    let mut buf = [0; 1024];
    let n = tls_stream.read(&mut buf)?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
