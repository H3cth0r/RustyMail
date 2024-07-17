use crate::imap::responses::IMAPResponse;

pub fn handle_command(command: &str) -> IMAPResponse {
    match command.trim() {
        "LOGIN"     => IMAPResponse::new("OK", "Logged in"),
        "SELECT"    => IMAPResponse::new("OK", "Mailbox selected"),
        "FETCH"     => IMAPResponse::new("OK", "Message fetched"),
        "LOGOUT"    => IMAPResponse::new("BYE", "Logged out"),
        _           => IMAPResponse::new("BAD", "Command unrecognized"),
    }
}
