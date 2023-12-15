use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use serde_json::Value;
use tauri::Wry;
use tauri_plugin_store::Store;

type StoreImpl = Store<Wry>;

lazy_static! {
    pub static ref TS_GEN_DIR: String = "tsGenDir".into();
    pub static ref SETTINGS_STORE: Arc<Mutex<Option<StoreImpl>>> = Arc::new(Mutex::new(None));
}

pub fn get_settings_value(s: String) -> Option<Value> {
    let lock = &mut *SETTINGS_STORE.lock().unwrap();
    let store = lock.as_mut().unwrap();
    store.load().unwrap();
    store.get(s).cloned()
}
