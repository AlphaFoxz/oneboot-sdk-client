///Sdk模块代码生成接口
pub mod sdk_gen_code_api {
    use super::super::super::super::super::apis_util;
    use super::super::super::enums::sdk_crud_service_type_enum;
    use crate::core::error::Error;

    ///创建单表CRUD代码
    ///_moduleName: 模块名称
    ///_poName: 表名
    ///_serviceType: 生成类型
    ///_force: 是否覆盖已有代码
    pub async fn generate_table_crud(
        _module_name: String,
        _po_name: String,
        _service_type: sdk_crud_service_type_enum::SdkCrudServiceTypeEnum,
        _force: bool,
    ) -> Result<bool, Error> {
        let __res = reqwest::Client::new()
            .get(
                apis_util::get_server_uri()
                    + "/_sdk/genCode/generateTableCrud?moduleName="
                    + _module_name.to_string().as_str()
                    + "&poName="
                    + _po_name.to_string().as_str()
                    + "&serviceType="
                    + Into::<i32>::into(_service_type).to_string().as_str()
                    + "&force="
                    + _force.to_string().as_str()
                    + "",
            )
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().text().await?.as_str().parse().unwrap())
    }
    ///创建整个模块的CRUD代码
    ///_moduleName: 模块名称
    ///_serviceType: 生成类型
    ///_force: 是否覆盖已有代码
    pub async fn generate_module_crud(
        _module_name: String,
        _service_type: sdk_crud_service_type_enum::SdkCrudServiceTypeEnum,
        _force: bool,
    ) -> Result<bool, Error> {
        let __res = reqwest::Client::new()
            .get(
                apis_util::get_server_uri()
                    + "/_sdk/genCode/generateModuleCrud?moduleName="
                    + _module_name.to_string().as_str()
                    + "&serviceType="
                    + Into::<i32>::into(_service_type).to_string().as_str()
                    + "&force="
                    + _force.to_string().as_str()
                    + "",
            )
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().text().await?.as_str().parse().unwrap())
    }
}
