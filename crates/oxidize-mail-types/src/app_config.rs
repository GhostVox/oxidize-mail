
pub struct AppConfig {
    db: String
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db: "oxidize_mail.db".to_string(),
        }
    }
}
impl AppConfig {
    pub fn get_db(&self) -> String {
        self.db.clone()
    }
}