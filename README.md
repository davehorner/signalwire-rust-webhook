webhook using salvo and lettre to send SMS messages via sendgrid.net.

Can be used with any SMTP server.

rename config.toml.rename to config.toml.

set your api key, from email set to a verified email registered on sendgrid, change To to the email to SMS address for your carrier.

To prevent unauthorized usage, set your account_sid to the signalwire account_sid that is sent along with the webhook request.  If you don't know it, run it once, it will output the sid, copy and save.

cargo run --release


point signalwire to your webhook.


