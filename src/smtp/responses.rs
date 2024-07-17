pub struct SMTPResponse {
    code: u16,
    message: String,
}
impl SMTPResponse {
    pub fn new(code: u16, message: &str) -> Self {
        SMTPResponse {
            code,
            message: message.to_string(),
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} {}\r\n", self.code, self.message)
    }
}
