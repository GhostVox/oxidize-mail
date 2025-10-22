pub use mail_parser::{Header, HeaderName, MessageParser};
use thiserror::Error;
use oxidize_mail_types::{ParsedEmail};

/// Represents errors encountered during parsing operations.
///
/// `ParseError` is an enumeration of potential errors that can occur while
/// performing parsing tasks. Each variant contains additional context about
/// the specific error.
///
/// # Variants
///
/// - `InvalidFormat(String)`
///   Indicates that the input data does not conform to the expected format.
///   For example, an invalid email address format. The associated `String`
///   provides details about the invalid input.
///
/// - `UnsupportedFeature(String)`
///   Signifies that a feature or behavior required during parsing
///   is not currently supported. The associated `String` provides
///   a description of the unsupported feature.
///
/// - `Other(String)`
///   Represents other types of parsing errors that do not fall under
///   `InvalidFormat` or `UnsupportedFeature`. The associated `String`
///   contains the error details.
///
/// # Example
///
/// ```
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// pub enum ParseError {
///     #[error("Invalid email format: {0}")]
///     InvalidFormat(String),
///     #[error("Unsupported feature: {0}")]
///     UnsupportedFeature(String),
///     #[error("Other parsing error: {0}")]
///     Other(String),
/// }
///
/// // Example of creating a ParseError
/// let error = ParseError::InvalidFormat("example@.com".to_string());
/// println!("{}", error);
/// ```
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid email format: {0}")]
    InvalidFormat(String),
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    #[error("Other parsing error: {0}")]
    Other(String),
}


/// `ParseService` is a struct that encapsulates a `MessageParser`
/// to handle parsing operations. It derives `PartialEq` to allow
/// for equality comparisons, `Debug` to enable debug formatting,
/// and `Default` to provide a default instance.
///
/// # Fields
/// - `parser`: An instance of `MessageParser` used to process and parse input data.
///
/// # Examples
/// ```
/// use oxidize_mail_parser::ParseService;
/// // Create a default instance of ParseService
/// let parse_service = ParseService::default();
///
/// // Compare instances (if applicable)
/// let another_service = ParseService::default();
/// assert_eq!(parse_service, another_service);
/// ```
///
/// Use this struct to interact with the underlying `MessageParser`.
#[derive(PartialEq, Debug, Default)]
pub struct ParseService {
    parser: MessageParser,
}

impl ParseService {
    pub fn new() -> ParseService {
        ParseService {
            parser: MessageParser::default(),
        }
    }

    pub fn parse_inc_email(&self, raw_email: &[u8]) -> Result<ParsedEmail, ParseError> {
        match self.parser.parse(raw_email) {
            Some(msg) => Ok(ParsedEmail::from_message(msg)),
            None => Err(ParseError::InvalidFormat(
                "Failed to parse email".to_string(),
            )),
        }
    }
}

fn parse_email(raw: &[u8]) -> Result<ParsedEmail, ParseError> {
    let service = ParseService::new();
    service.parse_inc_email(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_email() {
        let raw = b"From: test@example.com\r\nSubject: Hello\r\n\r\nThis is the body.";
        let result = parse_email(raw);
        assert!(result.is_ok());
        let email = result.unwrap();

        assert_eq!(email.subject.as_deref(), Some("Hello"));
        assert_eq!(email.from.as_deref(), Some("test@example.com"));
        assert_eq!(email.body_text.as_deref(), Some("This is the body."));
    }

    #[test]
    fn test_parse_invalid_email() {
        let raw = b"This is not a valid email at all.";
        let result = parse_email(raw);
        println!("{:?}", result);
        assert!(result.as_ref().unwrap().subject.is_none());
        assert!(result.as_ref().unwrap().from.is_none());
        assert!(result.as_ref().unwrap().body_text.is_none());
        assert!(result.unwrap().body_html.is_none());
    }
}
