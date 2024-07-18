extern crate clap;
extern crate num_cpus;
extern crate threadpool;

use clap::{App, Arg};
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

mod smtp;

fn parse_args() -> String {
    const BIND_HOST_ARG_NAME: &str = "host";
    const BIND_PORT_PORT_NAME: &str = "port";

    let matches = App::new("Rust SMTP server")
        .version("1.0")
        .author("h3cth0r")
        .about("Rusty SMTP server")
        .arg(
            Arg::with_name(BIND_HOST_ARG_NAME)
                .short("h")
                .help("Bind host")
                .default_value("localhost"),
        )
        .arg(
            Arg::with_name(BIND_PORT_PORT_NAME)
                .short("p")
                .help("Bind Port")
                .default_value("2525")
                .validator(|s: String| -> Result<(), String> {
                    s.parse::<u16>()
                        .and(Ok(()))
                        .map_err(|e: std::num::ParseIntError| -> String { e.to_string() })
                })
        ).get_matches();

        format!(
            "{}:{}",
            matches.value_of(BIND_HOST_ARG_NAME).unwrap(),
            matches.value_of(BIND_PORT_PORT_NAME).unwrap().to_string()
        )
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    match smtp::Connection::handle(&mut reader, &mut stream){
        Ok(result) => {
            println!("Sender domain: {}", result.get_sender_domain().unwrap());
            for message in result.get_messages().unwrap() {
                println!("Message from: {}", message.get_sender());
                println!("To: {}", message.get_recipients().join(", "));
                println!("{}", message.get_data());
            }
        }
        Err(e) => eprintln!("Error communicating with client {}", e),
    }
}

fn main() {
    let bind_address = parse_args();
    let listener = TcpListener::bind(&bind_address)
        .unwrap_or_else(|e| panic!("Binding to {} failed: {}", &bind_address, e));

    // Set parallel workers for incoming connections
    // Number of workers are equal to the number of cores
    let pool = ThreadPool::new(num_cpus::get());
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(e) => eprintln!("Unable to handle client connection: {}", e),
        }
    }
}
