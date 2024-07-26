use std::net::TcpStream;
use std::io::{ BufReader, BufWriter, Write, BufRead, Result, Error};
use crate::commands::*;
use crate::smtp::Message;

pub struct SmtpClient {
    state:              ClientState,
    reader:             BufReader<TcpStream>,
    writer:             BufWriter<TcpStream>,
    message:            Message,
    current_recipient:  usize,
}
impl SmtpClient {
    pub fn new(stream: TcpStream, message: Message) -> std::io::Result<Self> {
        // pub fn new(stream: TcpStream, message: Message) -> Result<Self> {
        let reader  = BufReader::new(stream.try_clone()?);
        let writer  = BufWriter::new(stream);

        Ok(SmtpClient {
            state:  ClientState::Connect,
            reader,
            writer,
            message,
            current_recipient: 0,
        })
    }
    pub fn send_email(&mut self) -> Result <()> {
        while self.state != ClientState::Done { self.process_state()?; }
        Ok(())
    }
    fn process_state(&mut self) -> Result<()> {
        match self.state {
            ClientState::Connect    => self.handle_connect(),
            ClientState::Helo       => self.handle_helo(),
            ClientState::MailFrom   => self.handle_mail_from(),
            ClientState::RcptTo     => self.handle_rcpt_to(),
            ClientState::Data       => self.handle_data(),
            ClientState::Content    => self.handle_content(),
            ClientState::Quit       => self.handle_quit(),
            ClientState::Done       => Ok(()),
        }
    }
    fn read_response(&mut self) -> Result<String> {
        let mut response = String::new();
        self.reader.read_line(&mut response)?;
        Ok(response.trim().to_string())
    }
    fn send_command(&mut self, command: &str) -> Result<()> {
        writeln!(self.writer, "{}", command)?;
        self.writer.flush()
    }
    fn handle_connect(&mut self) -> Result<()> {
        let response = self.read_response()?;
        if !response.starts_with("220") { return Err(Error::new(std::io::ErrorKind::Other, "Unexpected server greeting")); }
        self.state = ClientState::Helo;
        Ok(())
    }
    fn handle_helo(&mut self) -> Result<()> {
        self.send_command(&HELO_START_CMD("mx.zentinel.com"))?;  // TODO Check this domain
        let response = self.read_response()?;
        if !response.starts_with("250") { return Err(Error::new(std::io::ErrorKind::Other, "HELO command failed")); }
        self.state = ClientState::MailFrom;
        Ok(())
    }
    fn handle_mail_from(&mut self) -> Result<()> {
        self.send_command(&MAIL_FROM_CMD(self.message.get_sender()))?;
        let response = self.read_response()?;
        if !response.starts_with("250") { return Err(Error::new(std::io::ErrorKind::Other, "MAIL FROM command failed")); }
        self.state = ClientState::RcptTo;
        Ok(())
    }
    fn handle_rcpt_to(&mut self) -> Result<()> {
        if self.current_recipient < self.message.get_recipients().len() {
            let recipient   = &self.message.get_recipients()[self.current_recipient];
            self.send_command(&RCPT_TO_CMD(recipient))?;
            let response = self.read_response()?;
            if !response.starts_with("250") { return Err(Error::new(std::io::ErrorKind::Other, "RCPT TO command failed")); }
            self.current_recipient += 1;
        } else { self.state = ClientState::Data }
        Ok(())
    }
    fn handle_data(&mut self) -> Result<()> {
        self.send_command(DATA_CMD)?;
        let response = self.read_response()?;
        if !response.starts_with("354") { return Err(Error::new(std::io::ErrorKind::Other, "DATA command failed")); }
        self.state = ClientState::Content;
        Ok(())
    }
    fn handle_content(&mut self) -> Result<()> {
        for line in self.message.get_data().lines() { writeln!(self.writer, "{}", line)? }
        self.send_command(DOT_CMD)?;
        let response = self.read_response()?;
        if !response.starts_with("250") { return Err(Error::new(std::io::ErrorKind::Other, "Message content not accepted")); }
        self.state = ClientState::Quit;
        Ok(())
    }
    fn handle_quit(&mut self) -> Result<()> {
        self.send_command(QUIT_CMD)?;
        let response = self.read_response()?;
        if !response.starts_with("221") { return Err(Error::new(std::io::ErrorKind::Other, "QUIT command failed")); }
        self.state = ClientState::Done;
        Ok(())
    }
    
}
pub fn send_to_remote_smtp(message: &Message, remote_server: &str) -> std::io::Result<()> {
    println!("{}", remote_server);
    let stream = TcpStream::connect(remote_server)?;
    let mut client = SmtpClient::new(stream, message.to_owned())?;
    client.send_email()
}
