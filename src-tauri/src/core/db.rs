use crate::core::error::*;

///设置
pub static OPTION_KEY: &str = "OPTIONS";
///后端Host
pub static OPTION_BACKEND_HOST_KEY: &str = "OPTIONS:BACKEND_HOST";
///后端端口
pub static OPTION_BACKEND_PORT_KEY: &str = "OPTIONS:BACKEND_PORT";
///ts生成目录
pub static OPTION_TS_GEN_DIR_KEY: &str = "OPTIONS:TS_GEN_DIR";

fn get_db() -> Result<sled::Db> {
    let db = sled::open("./.sdk_db");
    if db.is_ok() {
        return Ok(db.unwrap());
    }
    Err(db.err().unwrap().into())
}
fn get_option_tree() -> Result<(sled::Tree, sled::Db)> {
    let db = get_db()?;
    get_option_tree_with_db(db)
}
fn get_option_tree_with_db(db: sled::Db) -> Result<(sled::Tree, sled::Db)> {
    let tree = db.open_tree(OPTION_KEY);
    if tree.is_ok() {
        return Ok((tree.unwrap(), db));
    }
    Err(tree.err().unwrap().into())
}
/// 初始化数据库里面的值
pub fn init_values() -> Result<()> {
    let (tree, _) = get_option_tree()?;
    match tree.get(OPTION_BACKEND_HOST_KEY)? {
        None => {
            tree.insert(OPTION_BACKEND_HOST_KEY, "127.0.0.1")?;
        }
        _default => {}
    }
    match tree.get(OPTION_BACKEND_PORT_KEY)? {
        None => {
            tree.insert(OPTION_BACKEND_PORT_KEY, "8080")?;
        }
        _default => {}
    }
    Ok(())
}

pub fn get_option_tree_value(key: &str) -> Result<String> {
    let (tree, _) = get_option_tree()?;
    let value = tree.get(key);
    if value.is_err() {
        return Err(value.err().unwrap().into());
    }
    let value = value.unwrap();
    if value.is_none() {
        return Ok(String::from(""));
    }
    Ok(String::from_utf8(value.unwrap().to_vec().into()).unwrap())
}

pub fn set_option_tree_value(key: &str, value: &str) -> Result<()> {
    let (tree, _) = get_option_tree()?;
    let result = tree.insert(key, value.as_bytes());
    if result.is_ok() {
        return Ok(());
    }
    Err(result.err().unwrap().into())
}

#[tauri::command]
pub fn get_option_tree_value_command(key: &str) -> Option<String> {
    let result = get_option_tree_value(key);
    if result.is_ok() {
        return Some(result.unwrap());
    }
    None
}

#[tauri::command]
pub fn set_option_tree_value_command(key: &str, value: &str) -> bool {
    let result = set_option_tree_value(key, value);
    result.is_ok()
}
