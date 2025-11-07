//! Email parsing utilities for the Oxidize Mail application.
//!
//! This crate provides high-level email parsing functionality built on top of the
//! `mail_parser` library. It offers a structured approach to parsing raw email data
//! into usable Rust types with comprehensive error handling.
//!
//! # Features
//!
//! - Parse raw email bytes into structured `ParsedEmail` objects
//! - Extract common email components (subject, from, to, body content)
//! - Comprehensive error handling with descriptive error types
//! - Support for both plain text and HTML email content
//!
//! # Example
//!
//! ```rust
//! use oxidize_mail_parser::{ParseService, ParseError};
//!
//! let raw_email = b"From: sender@example.com\r\nSubject: Hello\r\n\r\nEmail body content";
//! let service = ParseService::new();
//!
//! match service.parse_inc_email(raw_email) {
//!     Ok(email) => {
//!         println!("Subject: {:?}", email.subject);
//!         println!("From: {:?}", email.from);
//!     }
//!     Err(ParseError::InvalidFormat(msg)) => {
//!         eprintln!("Failed to parse email: {}", msg);
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```

pub use mail_parser::{Header, HeaderName, MessageParser};
use oxidize_mail_types::ParsedEmail;
use thiserror::Error;

/// Represents errors encountered during email parsing operations.
///
/// `ParseError` is an enumeration of potential errors that can occur while
/// performing email parsing tasks. Each variant contains additional context about
/// the specific error encountered. This error type implements the `Error` trait
/// from thiserror for better error handling and display formatting.
///
/// # Variants
///
/// - `InvalidFormat(String)`
///   Indicates that the input email data does not conform to the expected RFC format.
///   For example, malformed headers, invalid email addresses, or corrupted message structure.
///   The associated `String` provides details about the invalid input.
///
/// - `UnsupportedFeature(String)`
///   Signifies that a feature or behavior required during parsing
///   is not currently supported by this parser implementation. The associated `String`
///   provides a description of the unsupported feature.
///
/// - `Other(String)`
///   Represents other types of parsing errors that do not fall under
///   the specific categories above. The associated `String` contains detailed
///   error information for debugging purposes.
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_parser::ParseError;
/// use thiserror::Error;
///
/// // Example of creating different ParseError variants
/// let format_error = ParseError::InvalidFormat("Missing required header".to_string());
/// let unsupported_error = ParseError::UnsupportedFeature("MIME multipart/mixed".to_string());
/// let other_error = ParseError::Other("Unknown parsing failure".to_string());
///
/// // Display the error
/// println!("Parse error: {}", format_error);
///
/// // Use with Result types
/// fn parse_operation() -> Result<(), ParseError> {
///     Err(ParseError::InvalidFormat("Bad email format".to_string()))
/// }
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

/// A service struct that provides email parsing functionality using an underlying `MessageParser`.
///
/// `ParseService` encapsulates a `mail_parser::MessageParser` instance to handle
/// email parsing operations with a clean, high-level API. It converts raw email
/// data into structured `ParsedEmail` objects and handles error cases gracefully.
///
/// The struct derives `PartialEq` for equality comparisons, `Debug` for debugging
/// output, and `Default` to provide convenient instantiation.
///
/// # Fields
/// - `parser`: An instance of `mail_parser::MessageParser` used to process raw email data
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_parser::{ParseService, ParseError};
///
/// // Create a default instance of ParseService
/// let parse_service = ParseService::default();
///
/// // Parse a simple email
/// let raw_email = b"From: sender@example.com\r\nSubject: Test\r\n\r\nHello world!";
/// match parse_service.parse_inc_email(raw_email) {
///     Ok(parsed_email) => {
///         println!("Subject: {:?}", parsed_email.subject);
///         println!("From: {:?}", parsed_email.from);
///     }
///     Err(e) => eprintln!("Parse error: {}", e),
/// }
///
/// // Create a new instance explicitly
/// let another_service = ParseService::new();
/// assert_eq!(parse_service, another_service);
/// ```
///
/// This service provides a convenient interface for email parsing operations
/// while abstracting away the complexity of the underlying parser implementation.
#[derive(PartialEq, Debug, Default)]
pub struct ParseService {
    parser: MessageParser,
}

impl ParseService {
    /// Creates a new ParseService instance with a default MessageParser.
    ///
    /// This constructor method initializes a new ParseService with an underlying
    /// MessageParser configured with default settings. The service is ready to
    /// parse email data immediately after creation.
    ///
    /// # Returns
    ///
    /// A new ParseService instance ready for email parsing operations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_parser::ParseService;
    ///
    /// let service = ParseService::new();
    /// let raw_email = b"From: test@example.com\r\nSubject: Hello\r\n\r\nBody content";
    /// let result = service.parse_inc_email(raw_email);
    /// ```
    pub fn new() -> ParseService {
        ParseService {
            parser: MessageParser::default(),
        }
    }

    /// Parses raw email data into a structured ParsedEmail object.
    ///
    /// This method takes raw email bytes and attempts to parse them into a
    /// structured ParsedEmail representation. It handles the parsing internally
    /// and converts any parsing failures into appropriate ParseError variants.
    ///
    /// # Arguments
    ///
    /// * `raw_email` - Byte slice containing the raw email data to parse
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * `Ok(ParsedEmail)` - Successfully parsed email data
    /// * `Err(ParseError)` - Parsing failure with error details
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_parser::{ParseService, ParseError};
    ///
    /// let service = ParseService::new();
    /// let raw_email = b"From: sender@example.com\r\nSubject: Test\r\n\r\nHello world";
    ///
    /// match service.parse_inc_email(raw_email) {
    ///     Ok(email) => {
    ///         println!("Subject: {:?}", email.subject);
    ///         println!("From: {:?}", email.from);
    ///     }
    ///     Err(ParseError::InvalidFormat(msg)) => {
    ///         eprintln!("Parse error: {}", msg);
    ///     }
    ///     Err(e) => eprintln!("Other error: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `ParseError::InvalidFormat` if the email data cannot be parsed
    /// by the underlying MessageParser.
    //FIXME: The hardcoded values in ParsedEmail::from_message should be replaced with actual data.
    pub fn parse_inc_email(&self, raw_email: &[u8]) -> Result<ParsedEmail, ParseError> {
        match self.parser.parse(raw_email) {
            Some(msg) => Ok(ParsedEmail::from_message(
                msg, // here to 5 lines below
                false,
                "hello".to_string(),
                false,
                None,
            )),
            None => Err(ParseError::InvalidFormat(
                "Failed to parse email".to_string(),
            )),
        }
    }
}

/// Parses raw email data using a new ParseService instance.
///
/// This is a convenience function that creates a ParseService instance and
/// uses it to parse the provided raw email data. It's primarily used for
/// simple one-off parsing operations and in test cases.
///
/// # Arguments
///
/// * `raw` - Byte slice containing the raw email data to parse
///
/// # Returns
///
/// A Result containing either a ParsedEmail on success or a ParseError on failure
///
/// # Examples
///
/// ```rust
/// let raw_email = b"From: test@example.com\r\nSubject: Hello\r\n\r\nBody text";
/// let result = parse_email(raw_email);
/// assert!(result.is_ok());
/// ```
///
/// # Note
///
/// For better performance when parsing multiple emails, consider creating
/// a single ParseService instance and reusing it rather than calling this
/// function repeatedly.
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
