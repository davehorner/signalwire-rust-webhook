### signalwire-rust-webhook

signalwire webhook using salvo and lettre to send SMS messages via sendgrid.net and carrier email to SMS gateway.

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

A list for your convenience. Mileage may vary.
```
Email to SMS Gateway List
Carrier	Email-to-SMS	Email-to-MMS
Google Fi 1XXXXXXXXXX@msg.fi.google.com
AT&T	number@txt.att.net	number@mms.att.net
Sprint	number@pm.sprint.com	number@pm.sprint.com
T-Mobile	number@tmomail.net	number@tmomail.net
Verizon Wireless	number@vtext.com	number@vzwpix.com
Virgin Mobile	number@vmobl.com	number@vmobl.com
Sprint	number@messaging.sprintpcs.com	number@pm.sprint.com
AT&T Enterprise Paging	number@page.att.net	number@page.att.net
Rogers Wireless	number@sms.rogers.com	number@pcs.rogers.com
Boost Mobile	number@sms.myboostmobile.com	number@myboostmobile.com
Telus Mobility	number@msg.telus.com	number@msg.telus.com
Airfire Mobile	number@sms.airfiremobile.com	number@sms.airfire.ca
Ameritech	number@paging.acswireless.com	number@paging.acswireless.com
Assurance Wireless	number@vmobl.com	number@vmobl.com
BellSouth	number@bellsouth.cl	number@bellsouth.cl
Bluegrass Cellular	number@sms.bluecell.com	number@sms.bluecell.com
Cellcom	number@cellcom.quiktxt.com	number@cellcom.quiktxt.com
Cellular South	number@csouth1.com	number@csouth1.com
Chariton Valley Wireless	number@sms.cvalley.net	number@sms.cvalley.net
Chat Mobility	number@mail.msgsender.com	number@mail.msgsender.com
Cleartalk	number@sms.cleartalk.us	number@sms.cleartalk.us
Consumer Cellular	number@cingularme.com	number@mailmymobile.net
Consumer Cellular	number@mailmymobile.net	number@txt.att.net
Cricket	number@sms.cricketwireless.net	number@mms.cricketwireless.net
Element Mobile	number@SMS.elementmobile.net	number@myelementmobile.com
Esendex	number@echoemail.net	number@echoemail.net
Mellon Mobile	number@mellonmobile.ga	number@mellonmobile.com
MetroPCS	number@mymetropcs.com	number@mymetropcs.com
Nextech	number@sms.ntwls.net	number@sms.nextechwireless.com
Page Plus Cellular(Verizon MVNO)	number@vtext.com	number@vtext.com
South Central Communications	number@rinasms.com	number@rinasms.com
Southernlinc	number@page.southernlinc.com	number@page.southernlinc.com
Straight Talk	number@txt.att.net	number@mypixmessages.com
Syringa Wireless	number@rinasms.com	number@vtext.com
Teleflip	number@teleflip.com	number@rinasms.com
Union Wireless	number@union-tel.com	number@teleflip.com
US Cellular	number@email.uscc.net	number@union-tel.com
Voyager Mobile	number@text.voyagermobile.com	number@email.uscc.net
Centennial Wireless	number@cwemail.com	number@text.voyagermobile.com
TracFone (prepaid)	number@txt.att.net	number@cwemail.com
```
