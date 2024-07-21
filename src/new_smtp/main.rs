mod commands;
mod smtp;
mod server;
mod dns_lookup;

// fn main() {
//     let result = dns_lookup::lookup_mx_record("hector.miranda@zentinel.mx");
//     println!("{:?}", result);
// }

fn main() -> std::io::Result<()> {
    // server::start_server("127.0.0.1:2525")
    let result = dns_lookup::lookup_mx_record("hector.miranda@zentinel.mx");
    println!("{:?}", result);
}
