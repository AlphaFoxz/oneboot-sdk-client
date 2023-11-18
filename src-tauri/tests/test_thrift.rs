use oneboot_sdk_client_lib::core::error::*;
use oneboot_sdk_client_lib::core::util;
use oneboot_sdk_client_lib::thrift::{
    client::ThriftDefaultClient,
    gen::{dtos::sdk_request_dto, ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient},
};

#[test]
fn test_thrift_client() -> Result<()> {
    use oneboot_sdk_client_lib::thrift::client;

    let mut client = client::SdkThriftIfaceSyncClient::default_client().unwrap();
    let req_dto = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: "".into(),
    };
    let resp_dto = client.get_template_content_by_path(req_dto)?;
    eprint!("{:?}", resp_dto);
    Ok(())
}
