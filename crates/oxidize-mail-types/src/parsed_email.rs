/// Represents a parsed email with its main components.
///
/// The `ParsedEmail` struct is used to store the key components of an email
/// after it has been parsed. Each field in the struct is optional, allowing
/// for flexibility in cases where certain components of the email may not be
/// present or extracted during parsing.
///
/// # Fields
/// - `subject` (Option<String>): The subject line of the email, if available.
/// - `from` (Option<String>): The sender's email address, if available.
/// - `to` (Option<String>): The recipient's email address, if available.
/// - `body_text` (Option<String>): The plain-text body content of the email,
///    if available.
/// - `body_html` (Option<String>): The HTML body content of the email, if
///    available.
///
/// # Examples
///
/// ```
/// use oxidize_mail_parser::ParsedEmail;
///
/// let email = ParsedEmail {
///     subject: Some(String::from("Meeting Reminder")),
///     from: Some(String::from("sender@example.com")),
///     to: Some(String::from("recipient@example.com")),
///     body_text: Some(String::from("Don't forget about the meeting tomorrow.")),
///     body_html: None,
/// };
///
/// // Accessing fields
/// assert_eq!(email.subject.unwrap(), "Meeting Reminder");
/// assert_eq!(email.from.unwrap(), "sender@example.com");
/// ```
///
/// The `ParsedEmail` struct is useful for applications dealing with email
/// processing, such as email clients, parsers, or automation tools.
#[derive(PartialEq, Debug)]
pub struct ParsedEmail {
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
}

impl ParsedEmail {
    /// Constructs a `ParsedEmail` instance from a `mail_parser::Message`.
    ///
    /// This function extracts relevant data from the provided email message, such as
    /// the `subject`, `from` address, `to` address, and the first instances of
    /// text and HTML bodies, if available.
    ///
    /// # Parameters
    /// - `msg`: A `mail_parser::Message` representing the parsed representation of the email.
    ///
    /// # Returns
    /// A `ParsedEmail` instance containing:
    /// - `subject`: The subject of the email as an `Option<String>`. `None` if the subject is not present.
    /// - `from`: The sender's email address as an `Option<String>`. `None` if the sender is not present.
    /// - `to`: The recipient's email address as an `Option<String>`. `None` if the recipient is not present.
    /// - `body_text`: The plain text body of the email as an `Option<String>`. `None` if no plain text body is present.
    /// - `body_html`: The HTML body of the email as an `Option<String>`. `None` if no HTML body is present.
    ///
    /// # Usage
    /// ```
    /// use mail_parser::MessageParser;
    ///use oxidize_mail_parser::ParsedEmail;
    /// let raw_email = "..."; // Raw email string.
    /// let msgParser = MessageParser::default();
    /// let msg = msgParser.parse(raw_email.as_bytes()).expect("Failed to parse email");
    /// let msg = msgParser.parse(raw_email.as_bytes()).expect("Failed to parse email");
    /// let parsed_email = ParsedEmail::from_message(msg);
    ///
    /// assert!(parsed_email.subject.is_some());
    /// assert!(parsed_email.from.is_some());
    /// assert!(parsed_email.to.is_some());
    /// ```
    ///
    /// # Example Output
    /// Given a valid email message:
    /// ```
    /// Subject: "Hello, World!"
    /// From: "example@example.com"
    /// To: "recipient@example.com"
    /// ```
    /// The function will return:
    /// ```rust
    /// ParsedEmail {
    ///     subject: Some("Hello, World!".to_string()),
    ///     from: Some("example@example.com".to_string()),
    ///     to: Some("recipient@example.com".to_string()),
    ///     body_text: Some("...".to_string()), // if plain text exists
    ///     body_html: Some("...".to_string()), // if HTML exists
    /// }
    /// ```
    ///
    /// Note: All extracted data is optional and subject to the parsing result of `mail_parser::MessageParser`.
    pub fn from_message(msg: mail_parser::Message) -> ParsedEmail {
        let subject = msg.subject().map(|s| s.to_string());

        let from = msg
            .from()
            .and_then(|addr| addr.first())
            .and_then(|a| a.address())
            .map(|s| s.to_string());

        let to = msg
            .to()
            .and_then(|addr| addr.first())
            .and_then(|a| a.address())
            .map(|s| s.to_string());

        let body_text = msg.body_text(0).map(|b| b.to_string());
        let body_html = msg.body_html(0).map(|b| b.to_string());

        ParsedEmail {
            subject,
            from,
            to,
            body_text,
            body_html,
        }
    }

    // Helper function to get the body text
    pub fn body_text(&self, _max_length: usize) -> Option<String> {
        self.body_text.clone()
    }
}
