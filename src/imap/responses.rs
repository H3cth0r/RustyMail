pub struct IMAPResponse {
    status: String,
    message: String,
}

impl IMAPResponse {
    pub fn new(status: &str, message: &str) -> Self {
        IMAPResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}\r\n", self.status, self.message)
    }
}
