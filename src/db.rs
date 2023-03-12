//
// db.rs
//

pub mod db_connection_details {

    use crate::constants::constants::{
        DB_URL,
        ERROR_DB_CONNECTION,
    };

    #[derive(Debug)]
    pub struct DbConnectionDetails {
        pub url: &'static str,
        pub error: &'static str,
    }

    impl Default for DbConnectionDetails {
        fn default() -> Self {
            DbConnectionDetails {
                url: DB_URL,
                error: ERROR_DB_CONNECTION,
            }
        }
    }

    impl DbConnectionDetails {
        pub fn new() -> Self {
            Default::default()
        }
    }
}