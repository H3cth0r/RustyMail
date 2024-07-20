use crate::commands::*;
use std::io::{ BufRead, Error, Write };

pub struct Message {
    sender:     String,
    recipients: Vec<String>,
    data:       Vec<String>,
}
impl Message {
    pub fn get_sender(&self) -> &str { &self.sender }
    pub fn get_recipients(&self) -> &Vec<String> { &self.recipients }
    pub fn get_data(&self) -> String { self.data.join("\n") }
}

pub struct Connection {
    state:              State,
    sender_domain:      String,
    messages:           Vec<Message>,
    next_sender:        String,
    next_recipients:    Vec<String>,
    next_data:          Vec<String>,
}
impl Connection {
    pub fn new() -> Connection {
        Connection {
            state:          State::Helo,
            sender_domain:  "".to_string(),
            messages:       Vec::new(),
            next_sender:    "".to_string(),
            next_recipients:Vec::new(),
            next_data:      Vec::new(),
        }
    }

    pub fn feed_line(&mut self, line: &str) -> Result<String, String> {
        match self.state {
            State::Helo         => self.handle_helo(line),
            State::MailFrom     => self.handle_email_from(line),
            State::Rcpt         => self.handle_rcpt(line),
            State::Data         => self.handle_data(line),
            State::Dot          => self.handle_dot(line),
            State::Quit         => self.handle_quit(line),
        }
    }
    fn handle_helo(&mut self, line: &str) -> Result<String, String> {
        if line.starts_with("HELO ") {
            self.sender_domain  = line[5..].to_string();
            self.state          = State::MailFrom;
            Ok(MSG_OK.to_string())
        } else { Err(MSG_BAD_SEQUENCE.to_string()) }
    }
    fn handle_email_from(&mut self, line: &str) -> Result<String, String> {
        if line.starts_with("MAIL FROM:") {
            self.next_sender    = line[10..].to_string();
            self.state          = State::Rcpt;
            Ok(MSG_OK.to_string())
        } else { Err(MSG_BAD_SEQUENCE.to_string()) }
    }
    fn handle_rcpt(&mut self, line: &str) -> Result<String, String> {
        if line.starts_with("RCPT TO:") {
            self.next_recipients.push(line[8..].to_string());
            Ok(MSG_OK.to_string())
        } else if line == "DATA" {
            if self.next_recipients.is_empty() { Err(MSG_BAD_SEQUENCE.to_string()) }
            else {
                self.state = State::Data;
                Ok(MSG_START_MAIL_INPUT.to_string())
            }
        } else { Err(MSG_BAD_SEQUENCE.to_string()) }
    }
    fn handle_data(&mut self, line: &str) -> Result<String, String> {
        if line == "." {
            self.state = State::Dot;
            self.handle_dot(line)
        } else {
            self.next_data.push(line.to_string());
            Ok(String::new())
        }
    }
    fn handle_dot(&mut self, _line: &str) -> Result<String, String> {
        let message = Message {
            sender:     self.next_sender.clone(),
            recipients: self.next_recipients.clone(),
            data:       self.next_data.clone(),
        };
        self.messages.push(message);
        self.next_sender.clear();
        self.next_recipients.clear();
        self.next_data.clear();
        self.state = state::Quit; 
        Ok(MSG_OK.to_string())
    }
    fn handle_quit(&mut self, _line: &str) -> Result<String, String> { Ok(MSG_BYE.to_string()) }

    pub fn handle(reader: &mut dyn BufRead, writer: &mut dyn Write) -> Result<Connection, Error> {
        let mut connection = Connection::new();
        writeln!(writer, "{}", MSG_READY)?; // 220 ready. ? propagates error
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line = line.trim_end();

            match connection.feed_line(line) {
                Ok(response) if response.is_empty() => {}
                Ok(response) => {
                    writeln!(writer, "{}", response)?;
                    if response.starts_with("221") { break; }
                }
                Err(error) => writeln!(writer, "{}", response)?,
            }
        }
        Ok(connection)
    }
    fn get_if_done<R, F: FnOnce() -> R>(&self, getter: F) -> Option<R> {
        match self.state {
            State::Quit => Some(getter()),
            _ => None,
        }
    }
    pub fn get_messages(&self) -> Option<&Vec<Message>> { self.get_if_done(|| &self.messages) }
    pub fn get_sender_domain(&self) -> Option<&str> { self.get_if_done(|| self.sender_domain.as_str()) }

}
