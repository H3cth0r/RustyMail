// use crate::smtp::responses::SMTPResponse;

pub fn handle_command(command: &str) -> String {
    let command = command.trim().to_uppercase();
    if command.starts_with("HELO") || command.starts_with("EHLO") {
        return "250 Hello\r\n".to_string();
    }
    match &command[..] {
        s if s.starts_with("MAIL FROM:")    => "250 OK\r\n".to_string(),
        s if s.starts_with("RCPT TO:")      => "250 OK\r\n".to_string(),
        "DATA"                              => "354 End data with <CR><LF>.<CR><LF>\r\n".to_string(),
        "QUIT"                              => "221 Bye\r\n".to_string(),
        _                                   => "500 Command unrecognized\r\n".to_string(),
    }
}
