use oxidize_mail_storage::DB;
use oxidize_mail_types::AppConfig;
use std::panic;

pub struct CoreService {
    db: DB,
}

impl CoreService {
    pub fn new() -> Self {
        let app_config = AppConfig::default();
        let db = match DB::new(&app_config) {
            Ok(d) => d,
            Err(e) => panic!("Error connecting to database:{e}"),
        };
        Self { db: db }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn hello_test() {
        print!("hello")
    }
}
