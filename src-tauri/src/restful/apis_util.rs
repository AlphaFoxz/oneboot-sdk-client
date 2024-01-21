use crate::core::store;

pub fn get_server_uri() -> String {
    let host = store::get_settings_value(store::KEY_BACKEND_HOST.clone())
        .unwrap()
        .as_str()
        .unwrap_or("127.0.0.1")
        .to_string();
    let port = store::get_settings_value(store::KEY_BACKEND_PORT.clone())
        .unwrap()
        .as_i64()
        .unwrap_or(8080)
        .to_string();
    String::from(format!("http://{}:{}", host, port))
}
