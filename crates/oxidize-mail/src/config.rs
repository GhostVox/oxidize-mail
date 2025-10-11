use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ColorScheme {
    Light,
    Dark,
    System,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    preferred_color_scheme: ColorScheme,
    preferred_font: String,
    selected_folder: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            preferred_color_scheme: ColorScheme::Dark,
            preferred_font: String::from("Sans 11"),
            selected_folder: String::from("📥 All Inboxes"),
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = dirs::config_dir().unwrap().join("oxidize-mail/config.toml");

        if config_path.exists() {
            let contents = fs::read_to_string(config_path).expect("Failed to read config file");
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let config_path = dirs::config_dir().unwrap().join("oxidize-mail/config.toml");

        // Create parent directory if it doesn't exist
        fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");

        let contents = toml::to_string(self).expect("Failed to serialize config");
        fs::write(config_path, contents).expect("Failed to write config file");
    }

    pub fn get_preferred_color_scheme(&self) -> &ColorScheme {
        &self.preferred_color_scheme
    }
}
