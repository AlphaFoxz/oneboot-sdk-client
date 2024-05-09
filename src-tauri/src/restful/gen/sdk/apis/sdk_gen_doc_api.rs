///文档生成接口
pub mod sdk_gen_doc_api {
    use super::super::super::super::super::apis_util;
    use crate::core::error::Error;

    ///生成word Api文档
    ///_moduleName: 模块名称
    pub async fn generate_word_api(_module_name: String) -> Result<Vec<String>, Error> {
        let __res = reqwest::Client::new()
            .get(
                apis_util::get_server_uri()
                    + "/_sdk/genDoc/generateWordApi?moduleName="
                    + _module_name.to_string().as_str()
                    + "",
            )
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
}
