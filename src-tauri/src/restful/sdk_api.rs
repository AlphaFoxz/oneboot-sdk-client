use super::gen::sdk::apis::sdk_gen_code_api::sdk_gen_code_api;
use super::gen::sdk::apis::sdk_gen_doc_api::sdk_gen_doc_api;
use super::gen::sdk::enums::sdk_crud_service_type_enum;
use crate::core::error::Error;

#[tauri::command]
pub async fn generate_table_crud(
    module_name: String,
    po_name: String,
    r#type: sdk_crud_service_type_enum::SdkCrudServiceTypeEnum,
    force: bool,
) -> Result<bool, Error> {
    Ok(sdk_gen_code_api::generate_table_crud(module_name, po_name, r#type, force).await?)
}

#[tauri::command]
pub async fn generate_module_crud(
    module_name: String,
    r#type: sdk_crud_service_type_enum::SdkCrudServiceTypeEnum,
    force: bool,
) -> Result<bool, Error> {
    Ok(sdk_gen_code_api::generate_module_crud(module_name, r#type, force).await?)
}

#[tauri::command]
pub async fn generate_word_api(module_name: String) -> Result<String, Error> {
    Ok(sdk_gen_doc_api::generate_word_api(module_name)
        .await?
        .join(""))
}
