use crate::core::error::Error;

// TODO 结构化Response
pub async fn get_rpc_server_port(host: String) -> Result<u32, Error> {
    use crate::core::store;
    let http_port = store::get_settings_value(store::BACKEND_PORT.clone())
        .await
        .unwrap()
        .as_str()
        .unwrap_or("8080")
        .to_string();
    let url = format!("http://{}:{}/_sdk/thrift/getServerPort", host, http_port);
    let res: serde_json::Value = reqwest::get(url).await?.json().await?;
    let res = res.as_object().unwrap();
    let success = res.get("success").unwrap().as_bool().unwrap();
    if !success {
        return Err("获取RPC服务端口失败，请检查服务端错误信息".into());
    }
    Ok(res.get("data").unwrap().as_i64().unwrap() as u32)
}
