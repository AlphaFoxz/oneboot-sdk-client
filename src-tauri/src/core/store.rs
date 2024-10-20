use std::sync::{Arc, LazyLock, Mutex};

use serde_json::Value;
use tauri::Wry;
use tauri_plugin_store::Store;

type StoreImpl = Arc<Store<Wry>>;

pub static KEY_TS_CLIENT_GEN_DIR: LazyLock<String> = LazyLock::new(|| "tsClientGenDir".into());
pub static KEY_RUST_CLIENT_GEN_DIR: LazyLock<String> = LazyLock::new(|| "rustClientGenDir".into());
pub static KEY_BACKEND_HOST: LazyLock<String> = LazyLock::new(|| "backendHost".into());
pub static KEY_BACKEND_PORT: LazyLock<String> = LazyLock::new(|| "backendPort".into());
pub static SETTINGS_STORE: LazyLock<Arc<Mutex<Option<StoreImpl>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn get_settings_value(s: String) -> Option<Value> {
    let inner = &mut *SETTINGS_STORE.lock().unwrap();
    let store = inner.as_mut().unwrap();
    store.get(s).clone()
}
