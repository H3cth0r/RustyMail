mod commands;
mod smtp;
mod server;
mod dns_lookup;
mod smtp_client;
#[path = "../db/mod.rs"] mod db;
#[path = "../tests/mod.rs"] mod tests;
#[path = "../models/mod.rs"] mod models;

// fn main() {
//     let result = dns_lookup::lookup_mx_record("hector.miranda@zentinel.mx");
//     println!("{:?}", result);
// }

fn main() -> std::io::Result<()> {
    server::start_server("127.0.0.1:2525")
    // let result = dns_lookup::lookup_mx_record("hector.miranda@zentinel.mx");
    // println!("{:?}", result);
}


// #[tokio::main]
// async fn main() -> std::io::Result<()> {
    
//     server::start_server("127.0.0.1:2525").await
// }