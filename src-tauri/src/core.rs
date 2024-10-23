pub mod error;
pub mod io;
pub mod parser;
pub mod store;
pub mod strsim;

pub mod util {
    use snowflake::SnowflakeIdGenerator;
    use std::sync::{Arc, LazyLock, Mutex};

    pub static SNOWFLAKE: LazyLock<Arc<Mutex<SnowflakeIdGenerator>>> =
        LazyLock::new(|| Arc::new(Mutex::new(SnowflakeIdGenerator::new(0, 0))));
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
