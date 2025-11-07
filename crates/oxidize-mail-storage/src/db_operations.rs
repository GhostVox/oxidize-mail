pub(crate) const ADD_EMAIL: str = r#"
INSERT INTO emails (id, sender, recipient, subject, body, timestamp)
VALUES ($1, $2, $3, $4, $5, $6)
"#;

