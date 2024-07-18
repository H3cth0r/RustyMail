# Rusty Mail
Rust base email server

```
nc localhost 2525 <<EOT
HELO localhost
MAIL FROM: someone@localhost
RCPT TO: someone.else@localhost
DATA
Howdy,
the SMTP server works!
Bye.
.
QUIT
EOT
```
