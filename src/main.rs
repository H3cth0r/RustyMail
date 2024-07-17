mod smtp;

#[tokio::main]
async fn main() {
    smtp::server::run_smtp_server().await;
}
