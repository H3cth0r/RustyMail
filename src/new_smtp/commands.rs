// Page 29 RFC 821
pub let HELO_START_CMD:     |domain_t: &str|    = format!("HELO {}", domain_t);
pub let VERIFY_USER_CMD:    |username_t: &str|  = format!("VRFY {}", username_t);
pub let MAILING_LIST_CMD:   |group_name_t: &str|= format("EXPN {}", group_name_t)
pub let SEND_FROM_CMD:      |author_t: &str|    = format!("SEND FROM: {}", author_t);
pub let SOML_FROM_CMD:      |author_t: &str|    = format!("SOML FROM: {}", author_t);
pub let SAML_FROM_CMD:      |author_t: &str|    = format!("SAML FROM: {}", author_t);
pub let MAIL_FROM_CMD:      |author_t: &str|    = format!("MAIL FROM:{}", author_t);
pub let RCPT_TO_CMD:        |rcpt_t: &str|      = format!("RCPT TO:{}", rcpt_t);
pub const DATA_CMD:         &str                = "DATA";
pub const DOT_CMD:          &str                = ".";
pub const RSET_CMD:         &str                = "RSET";
pub const QUIT_CMD:         &str                = "QUIT";
pub const NOOP_CMD:         &str                = "NOOP";
pub const TURN_CMD:         &str                = "TURN";

pub enum State{
    Helo,
    Verify,
    MailingList,
    Send,
    Soml,
    Saml,
    MailFrom,
    Rcpt,
    Data,
    Dot,
    Rset,
    Quit,
    Noop,
    Turn,
}

// Server Response to client
pub const MSG_READY:                &str    = "220 ready";
pub const MSG_OK:                   &str    = "250 OK";
pub const MSG_SEND_MESSAGE_CONTENT: &str    = "354 Send message content";
pub const MSG_BYE:                  &str    = "221 Bye";
pub const MSG_SYNTAX_ERROR:         &str    = "500 unexpected line";

pub const MSG_SYSTEM_STATUS:        &str    = "211 System status, or system help reply";
pub const MSG_HELP:                 &str    = "214 Help message";
pub const MSG_SERVICE_READY:        &str    = "220 <domain> Service ready";
pub const MSG_SERVICE_CLOSING:      &str    = "221 <domain> Service closing transmission channel";
pub const MSG_REQUEST_OK:           &str    = "250 Requested mail action okay, completed";
pub const MSG_USER_NOT_LOCAL:       &str    = "251 User not local; will forward to <forward-path>";

pub const MSG_START_MAIL_INPUT:     &str    = "354 Start mail input; end with <CRLF>.<CRLF>";

pub const MSG_SERVICE_NOT_AVAILABLE:&str    = "421 <domain> Service not available, closing transmission channel";
pub const MSG_MAILBOX_UNAVAILABLE:  &str    = "450 Requested mail action not taken: mailbox unavailable";
pub const MSG_ACTION_ABORTED:       &str    = "451 Requested action aborted: local error in processing";
pub const MSG_INSUFFICIENT_STORAGE: &str    = "452 Requested action not taken: insufficient system storage";
 
pub const MSG_COMMAND_UNRECOGNIZED: &str    = "500 Syntax error, command unrecognized";
pub const MSG_PARAMETERS_ERROR:     &str    = "501 Syntax error in parameters or arguments";
pub const MSG_COMMAND_NOT_IMPLEMENTED: &str = "502 Command not implemented";
pub const MSG_BAD_SEQUENCE:         &str    = "503 Bad sequence of commands";
pub const MSG_PARAMETER_NOT_IMPLEMENTED: &str = "504 Command parameter not implemented";
pub const MSG_MAILBOX_UNAVAILABLE:  &str    = "550 Requested action not taken: mailbox unavailable";
pub const MSG_TRY_FORWARD_PATH:     &str    = "551 User not local; please try <forward-path>";
pub const MSG_STORAGE_EXCEEDED:     &str    = "552 Requested mail action aborted: exceeded storage allocation";
pub const MSG_MAILBOX_NAME_INVALID: &str    = "553 Requested action not taken: mailbox name not allowed";
pub const MSG_TRANSACTION_FAILED:   &str    = "554 Transaction failed";
