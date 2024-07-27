use std::io::BufReader as IoReader;
use std::fs::File;
use rustls::{ Certificate, PrivateKey, ServerConfig };
use std::io::Result;
use rustls_pemfile;

pub fn load_tls_config() -> Result<ServerConfig> {
    let cert_file   = &mut IoReader::new(File::open("path/to/cert.pem")?);
    let key_file    = &mut IoReader::new(File::open("path/to/key.pem")?);
    let cert_chain  = rustls_pemfile::certs(cert_file)?.into_iter().map(Certificate).collect();
    let mut keys    = rustls_pemfile::pkcs8_private_keys(key_file)?;
    let config      = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, PrivateKey(keys.remove(0)))?;
    Ok(config)
}
