pub mod error;
pub mod parser;
pub mod db;

pub mod util {
    use std::sync::{Arc, Mutex};

    use lazy_static::lazy_static;
    use snowflake::SnowflakeIdGenerator;
    lazy_static! {
        pub static ref SNOWFLAKE: Arc<Mutex<SnowflakeIdGenerator>> =
            Arc::new(Mutex::new(SnowflakeIdGenerator::new(0, 0)));
    }
    pub fn next_snowflake_id() -> i64 {
        let mut snowflake = *SNOWFLAKE.lock().unwrap();
        snowflake.generate()
    }

    #[cfg(test)]
    mod util_test {
        #[test]
        fn test_next_snowflake_id() {
            let id = super::next_snowflake_id();
            assert!(id > 0);
            eprintln!("{id}")
        }
    }
}
