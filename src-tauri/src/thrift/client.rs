use super::gen::ifaces;
use crate::core::error::Error;
use thrift::{
    protocol,
    transport::{self, TIoChannel},
};
pub type ClientInputProtocol = protocol::TBinaryInputProtocol<
    transport::TFramedReadTransport<transport::ReadHalf<transport::TTcpChannel>>,
>;
pub type ClientOutputProtocol = protocol::TMultiplexedOutputProtocol<
    protocol::TBinaryOutputProtocol<
        transport::TFramedWriteTransport<transport::WriteHalf<transport::TTcpChannel>>,
    >,
>;

async fn get_io_protocols(
    service_name: &str,
) -> Result<(ClientInputProtocol, ClientOutputProtocol), Error> {
    use crate::core::http;
    use crate::core::store;
    let mut c = transport::TTcpChannel::new();
    let backend_host =
        store::get_settings_value(store::KEY_BACKEND_HOST.clone()).unwrap_or("127.0.0.1".into());
    let backend_host = backend_host.as_str().unwrap_or("127.0.0.1");
    let rpc_server_port = http::get_rpc_server_port(backend_host.to_string())
        .await
        .unwrap_or(8081);
    let url = &format!("{}:{}", backend_host, rpc_server_port);
    c.open(url)?;
    let (i_chan, o_chan) = c.split()?;
    let i_prot =
        protocol::TBinaryInputProtocol::new(transport::TFramedReadTransport::new(i_chan), true);
    let o_prot = protocol::TMultiplexedOutputProtocol::<
        protocol::TBinaryOutputProtocol<
            transport::TFramedWriteTransport<transport::WriteHalf<transport::TTcpChannel>>,
        >,
    >::new(
        service_name,
        protocol::TBinaryOutputProtocol::new(transport::TFramedWriteTransport::new(o_chan), true),
    );
    Ok((i_prot, o_prot))
}

pub async fn try_get_sdk_info_client() -> Result<
    ifaces::sdk_info_iface::SdkInfoIfaceSyncClient<ClientInputProtocol, ClientOutputProtocol>,
    Error,
> {
    let (i_prot, o_prot) = get_io_protocols("sdkInfoIface").await?;
    Ok(ifaces::sdk_info_iface::SdkInfoIfaceSyncClient::new(
        i_prot, o_prot,
    ))
}

pub async fn try_get_sdk_thrift_client() -> Result<
    ifaces::sdk_thrift_iface::SdkThriftIfaceSyncClient<ClientInputProtocol, ClientOutputProtocol>,
    Error,
> {
    let (i_prot, o_prot) = get_io_protocols("sdkThriftIface").await?;
    Ok(ifaces::sdk_thrift_iface::SdkThriftIfaceSyncClient::new(
        i_prot, o_prot,
    ))
}

pub async fn try_get_sdk_gen_code_client() -> Result<
    ifaces::sdk_gen_code_iface::SdkGenCodeIfaceSyncClient<
        ClientInputProtocol,
        ClientOutputProtocol,
    >,
    Error,
> {
    let (i_prot, o_prot) = get_io_protocols("sdkGenCodeIface").await?;
    Ok(ifaces::sdk_gen_code_iface::SdkGenCodeIfaceSyncClient::new(
        i_prot, o_prot,
    ))
}

#[cfg(test)]
mod thrift_test {
    use crate::thrift::gen::ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient;

    #[tokio::test]
    pub async fn gen_code_client() {
        let c = super::try_get_sdk_gen_code_client().await;
        assert!(c.is_ok());
    }
    #[tokio::test]
    pub async fn thrift_client() {
        let c = super::try_get_sdk_thrift_client().await;
        assert!(c.is_ok());
        let mut c = c.unwrap();
        let result = c.get_executable_file_path();
        assert!(result.is_ok());
    }
}
