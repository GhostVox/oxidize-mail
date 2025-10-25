use std::path::PathBuf;
/// Application configuration settings for Oxidize Mail.
///
/// This struct contains core application configuration that determines
/// how the application connects to and manages its data storage. Currently
/// it focuses on database configuration but can be extended for other
/// application-wide settings.
///
/// # Fields
///
/// - `db` - The database file path or connection string
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_types::AppConfig;
///
/// // Create a default configuration
/// let config = AppConfig::default();
/// assert_eq!(config.get_db(), "oxidize_mail.db");
///
/// // Configuration is typically used to initialize database connections
/// ```
pub struct AppConfig {
    db_conn_string: PathBuf,
}

impl Default for AppConfig {
    /// Creates a default AppConfig with standard database settings.
    ///
    /// This implementation provides sensible defaults for the application
    /// configuration, using a local SQLite database file named "oxidize_mail.db"
    /// in the current working directory.
    ///
    /// # Returns
    ///
    /// An AppConfig instance with default values:
    /// * `db` - "oxidize_mail.db"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::AppConfig;
    ///
    /// let config = AppConfig::default();
    /// assert_eq!(config.get_db(), "oxidize.db");
    /// ```
    fn default() -> Self {
        Self {
            db_conn_string: dirs::config_dir()
                .unwrap()
                .join("oxidize-mail")
                .join("oxidize.db"),
        }
    }
}
impl AppConfig {
    pub fn new(conn_str: PathBuf) -> Self {
        Self {
            db_conn_string: conn_str,
        }
    }
    /// Returns the database file path or connection string.
    ///
    /// This method provides access to the configured database location,
    /// which is typically used when establishing database connections
    /// for email storage and retrieval operations.
    ///
    /// # Returns
    ///
    /// A `String` containing the database file path or connection string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::AppConfig;
    ///
    /// let config = AppConfig::default();
    /// let db_path = config.get_db();
    /// println!("Database location: {}", db_path);
    /// ```
    pub fn get_db(&self) -> PathBuf {
        self.db_conn_string.clone()
    }
}
