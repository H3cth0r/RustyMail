use rustls::{ ClientConfig, ClientConnection, StreamOwned };
use rustls_pemfile::{ certs, pkcs8_private_keys };
use std::io::{ BufReader, Read, Write };
use std::net::TcpStream;
use std::sync::Arc;

pub struct Relay {
    tls_config: Arc<ClientConfig>,
    username: String,
    password: String,
}
impl Relay {
    pub fn new(cert_path: &str, key_path: &str, username: String, password: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut root_cert_store     = rustls::RootCertStore::empty();
        root_cert_store.add_server_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .0
                .iter()
                .map(|ta| {
                    rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                })
        );

        let certs       = certs(&mut BufReader::new(std::fs::File::open(cert_path)?))?;
        let mut keys    = pkcs8_private_keys(&mut BufReader::new(std::fs::File::open(key_path)?))?;

        let config      = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_single_cert(certs, keys.remove(0))?;
        Ok(Relay, {
            tls_config: Arc::new(config),
            username,
            password,
        })
    }

    pub fn send_message(&self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        let remote_server   = lookup_mx_record(&message.recipients[0])?;
        let tcp_stream      = TcpStream::connect(remote_server)?;
        let domain          = remote_server.split(':').next().unwrap();
        let mut tls_conn    = ClientConnection::new(self.tls_config.clone(), domain.try_into()?)?;
        let mut tls_stream  ) StreamOwned::new(tls_conn, tcp_stream);

        self.smtp_ehlo(&mut tls_stream)?;
        self.smtp_auth(&mut tls_stream)?;
        self.smtp_mail_from(&mut tls_stream, &message.sender)?;
        for recipient in &message.recipients {
            self.smtp_rcpt_to(&mut tls_stream, recipient)?;
        }
        self.smtp_data(&mut tls_stream, &message.data)?;
        self.smtp_quit(&mut tls_stream)?;

        Ok(())
    }
}
