mod commands;
mod smtp;
mod server;

fn main() -> std::io::Result<()> {
    server::start_server("127.0.0.1:2525")
}
