### signalwire-rust-webhook

signalwire webhook using salvo and lettre to send SMS messages via sendgrid.net.

Can be used with any SMTP server.

rename config.toml.rename to config.toml.
```
smtp_relay = "smtp.sendgrid.net"
smtp_password = "<your api key>"
smtp_username = "apikey"
from_email = "verified@email_address.com"
to_email = "yourphonenumber@msg.fi.google.com"
webhook_path = "/webhook"
host = "0.0.0.0:7878"
email_subject = "New Webhook Message"
bcc_email = "bcc_recipient@example.com"
email_message_prefix = "New message received:\n"
account_sid = "your_account_sid from signalwire"
```
set smtp_password your sendgrid api key, set from_email to a verified email registered on sendgrid, change to_email to SMS address for your carrier.

cargo run --release


point signalwire to your webhook.


To prevent unauthorized usage account_sid is validated, set your account_sid to the signalwire account_sid that is sent along with the webhook request.  If you don't know it, run it once, it will output the sid, copy and save that in account_sid.
