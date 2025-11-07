/// Represents a parsed email with its main components.
///
/// The `ParsedEmail` struct is used to store the key components of an email
/// after it has been parsed. Each field in the struct is optional, allowing
/// for flexibility in cases where certain components of the email may not be
/// present or extracted during parsing.
///
/// # Fields
/// - `subject` (`Option<String>`): The subject line of the email, if available.
/// - `from` (`Option<String>`): The sender's email address, if available.
/// - `to` (`Option<String>`): The recipient's email address, if available.
/// - `body_text` (`Option<String>`): The plain-text body content of the email,
///    if available.
/// - `body_html` (`Option<String>`): The HTML body content of the email, if
///    available.
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_types::ParsedEmail;
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
    pub account_id: Option<String>,
    pub message_id: Option<String>,
    pub subject: Option<String>,
    pub sender: Option<String>,
    pub recipients: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub received_date: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub folder: Option<String>,
}

impl ParsedEmail {
    /// Constructs a `ParsedEmail` instance from a `mail_parser::Message`.
    ///
    /// This function extracts relevant data from the provided email message, including
    /// the subject line, sender and recipient addresses, and both plain text and HTML
    /// body content when available. It handles missing or malformed data gracefully
    /// by using `Option<String>` for all fields.
    ///
    /// # Arguments
    ///
    /// * `msg` - A `mail_parser::Message` representing the pre-parsed email structure
    ///
    /// # Returns
    ///
    /// A `ParsedEmail` instance containing:
    /// - `subject`: The email subject line as `Option<String>`, `None` if not present
    /// - `from`: The sender's email address as `Option<String>`, `None` if not present
    /// - `to`: The primary recipient's email address as `Option<String>`, `None` if not present
    /// - `body_text`: The plain text body content as `Option<String>`, `None` if not present
    /// - `body_html`: The HTML body content as `Option<String>`, `None` if not present
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mail_parser::MessageParser;
    /// use oxidize_mail_types::ParsedEmail;
    ///
    /// let raw_email = b"From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Hello World\r\n\r\nThis is the email body.";
    /// let parser = MessageParser::default();
    ///
    /// if let Some(msg) = parser.parse(raw_email) {
    ///     let parsed_email = ParsedEmail::from_message(msg);
    ///
    ///     assert_eq!(parsed_email.subject.as_deref(), Some("Hello World"));
    ///     assert_eq!(parsed_email.from.as_deref(), Some("sender@example.com"));
    ///     assert_eq!(parsed_email.to.as_deref(), Some("recipient@example.com"));
    ///     assert!(parsed_email.body_text.is_some());
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// - Only the first recipient in the "To" field is extracted
    /// - Only the first text and HTML body parts are extracted for simplicity
    /// - All fields are optional to handle incomplete or malformed emails gracefully
    pub fn from_message(
        msg: mail_parser::Message,
        seen: bool,
        folder: String,
        is_starred: bool,
        account_id: Option<String>,
    ) -> ParsedEmail {
        let subject = msg.subject().map(|s| s.to_string());

        let sender = msg
            .from()
            .and_then(|addr| addr.first())
            .and_then(|a| a.address())
            .map(|s| s.to_string());

        let recipients = msg
            .to()
            .and_then(|addr| addr.first())
            .and_then(|a| a.address())
            .map(|s| s.to_string());

        let body_text = msg.body_text(0).map(|b| b.to_string());
        let body_html = msg.body_html(0).map(|b| b.to_string());
        let timestamp = msg.date();
        let message_id = msg.message_id().map(|s| s.to_string());

        ParsedEmail {
            account_id,
            message_id,
            subject,
            sender,
            recipients,
            body_text,
            body_html,
            received_date: timestamp.map(|s| s.to_string()),
            is_read: seen,
            is_starred,
            folder: Some(folder),
        }
    }

    /// Returns a clone of the email's plain text body content.
    ///
    /// This is a convenience method that provides access to the plain text body
    /// of the email. The `max_length` parameter is currently unused but reserved
    /// for future functionality to truncate long email bodies.
    ///
    /// # Arguments
    ///
    /// * `_max_length` - Maximum length for the returned text (currently unused)
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the plain text body, or `None` if no text body exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::ParsedEmail;
    ///
    /// let email = ParsedEmail {
    ///     subject: Some("Test".to_string()),
    ///     from: Some("sender@example.com".to_string()),
    ///     to: Some("recipient@example.com".to_string()),
    ///     body_text: Some("Hello, world!".to_string()),
    ///     body_html: None,
    /// };
    ///
    /// let body = email.body_text(1000);
    /// assert_eq!(body.as_deref(), Some("Hello, world!"));
    /// ```
    pub fn body_text(&self, _max_length: usize) -> Option<String> {
        self.body_text.clone()
    }

    pub fn preview(&self) -> String {
        //generate a short preview of the body
        if let Some(body) = &self.body_text {
            let preview_length = 50;
            if body.len() > preview_length {
                format!("{}...", &body[..preview_length])
            } else {
                body.clone()
            }
        } else {
            String::from("(No body text)")
        }
    }

    //TODO: this is a standin, implement proper formatting
    pub fn time_string(&self) -> String {
        if let Some(timestamp) = &self.received_date {
            format!("{}", timestamp)
        } else {
            String::from("(No timestamp)")
        }
    }
}
