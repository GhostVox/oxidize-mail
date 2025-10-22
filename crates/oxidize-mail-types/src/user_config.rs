use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use dirs;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
    System,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    preferred_color_scheme: ColorScheme,
    preferred_font: String,
    selected_folder: String,

}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            preferred_color_scheme: ColorScheme::Dark,
            preferred_font: String::from("Sans 11"),
            selected_folder: String::from("📥 All Inboxes"),
        }
    }
}

impl UserConfig {
    pub fn load() -> Self {
        Self::load_from_path(dirs::config_dir().unwrap().join("oxidize-mail/config.toml"))
    }

    fn load_from_path<P: AsRef<Path>>(config_path: P) -> Self {
        if config_path.as_ref().exists() {
            let contents = fs::read_to_string(config_path).expect("Failed to read config file");
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let config_path = dirs::config_dir().unwrap().join("oxidize-mail/config.toml");

        // Create a parent directory if it doesn't exist
        fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");

        let contents = toml::to_string(self).expect("Failed to serialize config");
        fs::write(config_path, contents).expect("Failed to write config file");
    }

    pub fn get_preferred_color_scheme(&self) -> &ColorScheme {
        &self.preferred_color_scheme
    }

    pub fn update_selected_folder(&mut self, folder: &str) {
        self.selected_folder = folder.to_string();
    }
    pub fn get_selected_folder(&self) -> &str {
        &self.selected_folder
    }
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
