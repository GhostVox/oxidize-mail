-- Add folders table for better organization
CREATE TABLE IF NOT EXISTS folders
(
    id
    INTEGER
    PRIMARY
    KEY
    AUTOINCREMENT,
    account_id
    INTEGER
    NOT
    NULL,
    name
    TEXT
    NOT
    NULL,
    type
    TEXT
    NOT
    NULL, -- INBOX, SENT, DRAFTS, TRASH, CUSTOM
    unread_count
    INTEGER
    DEFAULT
    0,
    FOREIGN
    KEY
(
    account_id
) REFERENCES accounts
(
    id
),
    UNIQUE
(
    account_id,
    name
)
    );

-- Add folder_id reference to emails
ALTER TABLE emails
    ADD COLUMN folder_id INTEGER REFERENCES folders (id);