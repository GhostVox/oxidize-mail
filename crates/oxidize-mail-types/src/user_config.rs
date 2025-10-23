use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents the available color scheme options for the application theme.
///
/// This enum defines the three color scheme modes that users can select from
/// in the application settings. Each variant controls how the application
/// determines which theme to display.
///
/// # Variants
///
/// - `Light` - Forces the application to use light theme regardless of system preference
/// - `Dark` - Forces the application to use dark theme regardless of system preference
/// - `System` - Uses the system's preferred color scheme (light or dark)
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_types::ColorScheme;
/// use serde::{Serialize, Deserialize};
///
/// let light_scheme = ColorScheme::Light;
/// let dark_scheme = ColorScheme::Dark;
/// let system_scheme = ColorScheme::System;
///
/// // ColorScheme can be serialized/deserialized
/// let json = serde_json::to_string(&dark_scheme).unwrap();
/// let deserialized: ColorScheme = serde_json::from_str(&json).unwrap();
/// assert_eq!(deserialized, ColorScheme::Dark);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
    System,
}

/// User configuration settings for the Oxidize Mail application.
///
/// This struct holds all user-customizable settings that persist between
/// application sessions. The configuration is automatically serialized to
/// and deserialized from a TOML file in the user's configuration directory.
///
/// # Fields
///
/// - `preferred_color_scheme` - The user's preferred color scheme (Light, Dark, or System)
/// - `preferred_font` - Font family and size specification (e.g., "Sans 11")
/// - `selected_folder` - The currently selected email folder name
///
/// # Examples
///
/// ```rust
/// use oxidize_mail_types::{UserConfig, ColorScheme};
///
/// // Create a default configuration
/// let config = UserConfig::default();
/// assert_eq!(config.get_preferred_color_scheme(), &ColorScheme::Dark);
/// assert_eq!(config.get_selected_folder(), "📥 All Inboxes");
///
/// // Create a custom configuration
/// let mut custom_config = UserConfig::default();
/// custom_config.update_color_scheme(ColorScheme::Light);
/// custom_config.update_selected_folder("Work");
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    preferred_color_scheme: ColorScheme,
    preferred_font: String,
    selected_folder: String,
}

impl Default for UserConfig {
    /// Creates a default UserConfig with sensible application defaults.
    ///
    /// This implementation provides reasonable defaults for new users of the
    /// Oxidize Mail application. It sets up dark theme preference, standard
    /// system font, and selects the main inbox as the default folder.
    ///
    /// # Returns
    ///
    /// A UserConfig instance with default values:
    /// * `preferred_color_scheme` - ColorScheme::Dark
    /// * `preferred_font` - "Sans 11"
    /// * `selected_folder` - "📥 All Inboxes"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::{UserConfig, ColorScheme};
    ///
    /// let config = UserConfig::default();
    /// assert_eq!(config.get_preferred_color_scheme(), &ColorScheme::Dark);
    /// assert_eq!(config.get_selected_folder(), "📥 All Inboxes");
    /// ```
    fn default() -> Self {
        Self {
            preferred_color_scheme: ColorScheme::Dark,
            preferred_font: String::from("Sans 11"),
            selected_folder: String::from("📥 All Inboxes"),
        }
    }
}

impl UserConfig {
    /// Loads user configuration from the default system configuration directory.
    ///
    /// This method loads the configuration from the standard user config directory
    /// path: `~/.config/oxidize-mail/config.toml` on Unix systems, or the equivalent
    /// on other platforms. If the file doesn't exist or cannot be parsed, returns
    /// the default configuration.
    ///
    /// # Returns
    ///
    /// A `UserConfig` instance loaded from the file system, or default values if loading fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::UserConfig;
    ///
    /// let config = UserConfig::load();
    /// // Configuration is now loaded from ~/.config/oxidize-mail/config.toml
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the system configuration directory cannot be determined.
    pub fn load() -> Self {
        Self::load_from_path(dirs::config_dir().unwrap().join("oxidize-mail/config.toml"))
    }

    /// Loads user configuration from a specified file path.
    ///
    /// This method attempts to read and parse a TOML configuration file from the
    /// given path. If the file doesn't exist, cannot be read, or contains invalid
    /// TOML, it returns the default configuration instead of failing.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the configuration file to load
    ///
    /// # Returns
    ///
    /// A `UserConfig` instance with loaded settings, or default values if loading fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::UserConfig;
    /// use std::path::Path;
    ///
    /// let config = UserConfig::load_from_path(Path::new("/custom/path/config.toml"));
    /// ```
    fn load_from_path<P: AsRef<Path>>(config_path: P) -> Self {
        if config_path.as_ref().exists() {
            let contents = fs::read_to_string(config_path).expect("Failed to read config file");
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Saves the current configuration to the default system configuration directory.
    ///
    /// This method serializes the current configuration to TOML format and writes
    /// it to the standard user config directory. It creates the necessary parent
    /// directories if they don't exist.
    ///
    /// The file is saved to: `~/.config/oxidize-mail/config.toml` on Unix systems,
    /// or the equivalent path on other platforms.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::{UserConfig, ColorScheme};
    ///
    /// let mut config = UserConfig::default();
    /// config.update_color_scheme(ColorScheme::Light);
    /// config.save(); // Persists changes to disk
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The system configuration directory cannot be determined
    /// - The configuration directory cannot be created
    /// - The configuration cannot be serialized to TOML
    /// - The file cannot be written to disk
    pub fn save(&self) {
        let config_path = dirs::config_dir().unwrap().join("oxidize-mail/config.toml");

        // Create a parent directory if it doesn't exist
        fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");

        let contents = toml::to_string(self).expect("Failed to serialize config");
        fs::write(config_path, contents).expect("Failed to write config file");
    }

    /// Returns a reference to the user's preferred color scheme.
    ///
    /// This method provides read-only access to the current color scheme setting,
    /// which determines whether the application uses light, dark, or system theme.
    ///
    /// # Returns
    ///
    /// A reference to the current `ColorScheme` setting
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::{UserConfig, ColorScheme};
    ///
    /// let config = UserConfig::default();
    /// match config.get_preferred_color_scheme() {
    ///     ColorScheme::Light => println!("Using light theme"),
    ///     ColorScheme::Dark => println!("Using dark theme"),
    ///     ColorScheme::System => println!("Using system theme"),
    /// }
    /// ```
    pub fn get_preferred_color_scheme(&self) -> &ColorScheme {
        &self.preferred_color_scheme
    }

    /// Updates the currently selected email folder.
    ///
    /// This method changes which email folder is currently active in the application.
    /// The folder name is stored as a string and typically represents a folder like
    /// "Inbox", "Sent", "Drafts", etc.
    ///
    /// # Arguments
    ///
    /// * `folder` - The name of the folder to select
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::UserConfig;
    ///
    /// let mut config = UserConfig::default();
    /// config.update_selected_folder("📤 Sent");
    /// assert_eq!(config.get_selected_folder(), "📤 Sent");
    /// ```
    pub fn update_selected_folder(&mut self, folder: &str) {
        self.selected_folder = folder.to_string();
    }
    /// Returns the name of the currently selected email folder.
    ///
    /// This method provides read-only access to the currently active folder name,
    /// which represents which email folder the user has selected in the interface.
    ///
    /// # Returns
    ///
    /// A string slice containing the selected folder name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::UserConfig;
    ///
    /// let config = UserConfig::default();
    /// println!("Current folder: {}", config.get_selected_folder());
    /// ```
    pub fn get_selected_folder(&self) -> &str {
        &self.selected_folder
    }
    /// Updates the user's preferred color scheme setting.
    ///
    /// This method changes the application's color scheme preference, which determines
    /// whether the application displays in light mode, dark mode, or follows the
    /// system preference.
    ///
    /// # Arguments
    ///
    /// * `scheme` - The new color scheme to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use oxidize_mail_types::{UserConfig, ColorScheme};
    ///
    /// let mut config = UserConfig::default();
    /// config.update_color_scheme(ColorScheme::Light);
    /// assert_eq!(config.get_preferred_color_scheme(), &ColorScheme::Light);
    ///
    /// config.update_color_scheme(ColorScheme::System);
    /// assert_eq!(config.get_preferred_color_scheme(), &ColorScheme::System);
    /// ```
    pub fn update_color_scheme(&mut self, scheme: ColorScheme) {
        self.preferred_color_scheme = scheme;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = UserConfig::default();
        assert_eq!(config.preferred_color_scheme, ColorScheme::Dark);
        assert_eq!(config.preferred_font, "Sans 11");
        assert_eq!(config.selected_folder, "📥 All Inboxes");
    }

    #[test]
    fn test_get_preferred_color_scheme() {
        let config = UserConfig::default();
        assert_eq!(config.get_preferred_color_scheme(), &ColorScheme::Dark);
    }

    #[test]
    fn test_update_color_scheme() {
        let mut config = UserConfig::default();
        assert_eq!(config.preferred_color_scheme, ColorScheme::Dark);

        config.update_color_scheme(ColorScheme::Light);
        assert_eq!(config.preferred_color_scheme, ColorScheme::Light);

        config.update_color_scheme(ColorScheme::System);
        assert_eq!(config.preferred_color_scheme, ColorScheme::System);
    }

    #[test]
    fn test_load_from_path_config_exists() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create a test config file with valid AppConfig fields
        let test_config = r#"
preferred_color_scheme = "Light"
preferred_font = "Monospace 12"
selected_folder = "Work"
        "#;
        fs::write(&config_path, test_config).unwrap();

        // Load the config from the test path
        let config = UserConfig::load_from_path(&config_path);

        // Verify the config was loaded correctly
        assert_eq!(config.preferred_color_scheme, ColorScheme::Light);
        assert_eq!(config.preferred_font, "Monospace 12");
        assert_eq!(config.selected_folder, "Work");
    }

    #[test]
    fn test_load_from_path_config_not_exists() {
        // Create a temporary path that doesn't exist
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent-config.toml");

        // Load the config - should return default
        let config = UserConfig::load_from_path(&config_path);

        // Verify it's the default config
        assert_eq!(config.preferred_color_scheme, ColorScheme::Dark);
        assert_eq!(config.preferred_font, "Sans 11");
        assert_eq!(config.selected_folder, "📥 All Inboxes");
    }

    #[test]
    fn test_load_malformed_config_returns_default() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create a malformed config file
        let test_config = r#"
this is not valid toml!!!
        "#;
        fs::write(&config_path, test_config).unwrap();

        // Load the config - should return default due to unwrap_or_default()
        let config = UserConfig::load_from_path(&config_path);

        // Verify it's the default config
        assert_eq!(config.preferred_color_scheme, ColorScheme::Dark);
        assert_eq!(config.preferred_font, "Sans 11");
        assert_eq!(config.selected_folder, "📥 All Inboxes");
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("oxidize-mail").join("config.toml");

        // Create a custom config
        let original_config = UserConfig {
            preferred_color_scheme: ColorScheme::System,
            preferred_font: String::from("Arial 14"),
            selected_folder: String::from("Custom Folder"),
        };

        // Save it (we need to modify save() to accept a path, or test differently)
        // Since save() uses a hardcoded path, we'll test save/load separately

        // For now, let's manually save to our test path
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        let contents = toml::to_string(&original_config).unwrap();
        fs::write(&config_path, contents).unwrap();

        // Load it back
        let loaded_config = UserConfig::load_from_path(&config_path);

        // Verify they match
        assert_eq!(loaded_config.preferred_color_scheme, ColorScheme::System);
        assert_eq!(loaded_config.preferred_font, "Arial 14");
        assert_eq!(loaded_config.selected_folder, "Custom Folder");
    }

    #[test]
    fn test_all_color_schemes() {
        let temp_dir = TempDir::new().unwrap();

        // Test Light
        let config_path = temp_dir.path().join("light.toml");
        fs::write(
            &config_path,
            r#"
preferred_color_scheme = "Light"
preferred_font = "Sans 11"
selected_folder = "📥 All Inboxes"
    "#,
        )
        .unwrap();
        let config = UserConfig::load_from_path(&config_path);
        assert_eq!(config.preferred_color_scheme, ColorScheme::Light);

        // Test Dark
        let config_path = temp_dir.path().join("dark.toml");
        fs::write(
            &config_path,
            r#"
preferred_color_scheme = "Dark"
preferred_font = "Sans 11"
selected_folder = "📥 All Inboxes"
    "#,
        )
        .unwrap();
        let config = UserConfig::load_from_path(&config_path);
        assert_eq!(config.preferred_color_scheme, ColorScheme::Dark);

        // Test System
        let config_path = temp_dir.path().join("system.toml");
        fs::write(
            &config_path,
            r#"
preferred_color_scheme = "System"
preferred_font = "Sans 11"
selected_folder = "📥 All Inboxes"
    "#,
        )
        .unwrap();
        let config = UserConfig::load_from_path(&config_path);
        assert_eq!(config.preferred_color_scheme, ColorScheme::System);
    }
}
