use oxidize_mail_types::AppConfig;
use rusqlite::{Connection, Result};
use std::fs;

// Embed migrations at compile time
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    /// Creates a new database connection and runs migrations

    pub fn new(config: &AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = config.get_db();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| rusqlite::Error::InvalidPath(db_path.clone()))?;
        }

        let mut conn = Connection::open(config.get_db())?;

        // Run migrations
        embedded::migrations::runner()
            .run(&mut conn)
            .map_err(|e| format!("Migration failed: {}", e))?;

        Ok(Self { conn })
    }

    /// Get the current migration version
    pub fn migration_version(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let version = embedded::migrations::runner()
            .get_last_applied_migration(&mut self.conn)?
            .map(|m| m.version())
            .unwrap_or(0);
        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_migrations_run() {
        let temp = NamedTempFile::new().unwrap();
        let config = AppConfig::default();
        // Set to temp file path

        let mut db: DB = DB::new(&config).expect("Failed to create DB");
        let version = db.migration_version().expect("Failed to get version");
        assert!(version > 0, "Migrations should have run");
    }
}
