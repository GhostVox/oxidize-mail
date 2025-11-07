-- Initial email storage schema
CREATE TABLE IF NOT EXISTS accounts
(
id INTEGER PRIMARY KEY AUTOINCREMENT,
email TEXT NOT NULL UNIQUE,
provider TEXT NOT NULL,
created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) ;

CREATE TABLE IF NOT EXISTS emails
(
id INTEGER PRIMARY KEY AUTOINCREMENT,
account_id INTEGER NOT NULL,
message_id TEXT NOT NULL UNIQUE,
subject TEXT,
sender TEXT NOT NULL,
recipients TEXT,
body_text TEXT,
body_html TEXT,
received_date TIMESTAMP,
is_read BOOLEAN DEFAULT 0,
is_starred BOOLEAN DEFAULT 0,
folder TEXT DEFAULT 'INBOX',
FOREIGN KEY (account_id) REFERENCES accounts (id)
) ;

CREATE INDEX idx_emails_account ON emails (account_id) ;
CREATE INDEX idx_emails_folder ON emails (account_id, folder) ;
CREATE INDEX idx_emails_date ON emails (received_date DESC) ;
