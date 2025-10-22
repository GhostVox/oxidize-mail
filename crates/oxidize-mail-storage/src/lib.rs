use oxidize_mail_types::AppConfig;
use rusqlite::{params, Connection, Result};

struct DB {
    conn: Connection,
}

impl DB {
    pub fn new(config: &Config) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            conn: Connection::open(&config.app_db)?,
        })
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
