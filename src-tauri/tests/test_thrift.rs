use oneboot_sdk_client_lib::core::util;
use oneboot_sdk_client_lib::thrift::gen::{
    dtos::sdk_request_dto, ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient,
};

#[tokio::test]
async fn test_thrift_client() {
    use oneboot_sdk_client_lib::thrift::client;

    let mut client = client::try_get_sdk_thrift_client().await.unwrap();
    let req_dto = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: "".into(),
    };
    let resp_dto = client.get_template_content_by_path(req_dto).unwrap();
    eprint!("{:?}", resp_dto);
}
