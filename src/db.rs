//
// db.rs
//

use crate::constants::constants::{
    DB_URL,
};

#[derive(Debug)]
pub struct DbConnectionCredentials {
    pub url: String,
}

impl Default for DbConnectionCredentials {
    fn default() -> Self {
        DbConnectionCredentials {
            url: DB_URL.to_string(),
        }
    }
}

impl DbConnectionCredentials {
    pub fn new() -> Self {
        Default::default()
    }
}
