pub use mail_parser::{MessageParser,Header, HeaderName};
use thiserror::Error;
#[derive(Error, Debug)]
enum ParseError {
    #[error("Invalid email format: {0}")]
    InvalidFormat(String),
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    #[error("Other parsing error: {0}")]
    Other(String),
}
#[derive(PartialEq, Debug)]
struct ParsedEmail <'x>{
    subject: Option<String>,
    from: Option<mail_parser::Address<'x>>,
    to: Option<mail_parser::Address<'x>>,
    body_text: Option<String>,
    body_html: Option<String>,
}

impl ParsedEmail {
    pub fn from_message(msg: mail_parser::Message) -> ParsedEmail {
        let message_id_hash = msg.message_id();
        let subject = msg.get_subject().map(|s| s.to_string());
        let from = msg.from();
        let to = msg.to();
        let body_text = msg.text_body.map(|b| b.to_string());
        let body_html = msg.html_body.map(|b| b.to_string());

        ParsedEmail {
            subject,
            from,
            to,
            body_text,
            body_html,
        }
    }

    pub fn body_text(&self, _max_length: usize) -> Option<String> {
        self.body_text.clone()
    }
}

#[derive(PartialEq, Debug, Default)]
struct ParseService {
    parser: MessageParser,
}
impl ParseService {
    pub fn new() -> ParseService {
        ParseService {
            parser: MessageParser::new(),
        }
    }

    pub fn parse_inc_email(&self, raw_email: &[u8]) -> Result<ParsedEmail, ParseError> {
        let message = self.parser.parse(raw_email);
        match message {
            Ok(msg) => Ok(ParsedEmail::from_message(msg)),
            Err(e) => Err(ParseError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_email() {
        // A minimal, valid raw email
        let raw = b"From: test@example.com\r\nSubject: Hello\r\n\r\nThis is the body.";

        let result = parse_email(raw);
        assert!(result.is_ok());

        let email = result.unwrap();

        // Check that the parser extracted the correct information
        assert_eq!(email.subject.as_deref(), Some("Hello"));
        assert_eq!(
            email.from.as_ref().and_then(|h| h.get_first_address()),
            Some("test@example.com")
        );
        assert_eq!(email.body_text(0).as_deref(), Some("This is the body."));
    }

    #[test]
    fn test_parse_invalid_email() {
        // Just some random text
        let raw = b"This is not a valid email at all.";
        let result = parse_email(raw);

        // We expect this to fail parsing
        assert!(result.is_err());
    }
}
