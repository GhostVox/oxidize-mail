pub(crate) const ADD_EMAIL: &str = r#"
INSERT INTO emails (account_id, message_id, subject, sender, recipients, body_text, body_html, recieved_date, is_read, is_starred, folder)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
"#;
