use super::gen::ifaces;
use crate::core::error::*;
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

// TODO 尝试用宏简化
pub trait ThriftDefaultClient {
    fn default_client() -> Result<Self>
    where
        Self: Sized;
}

fn get_io_protocols(service_name: &str) -> Result<(ClientInputProtocol, ClientOutputProtocol)> {
    use crate::core::store;
    let mut c = transport::TTcpChannel::new();
    c.open(&format!(
        "{:?}:{:?}",
        store::get_settings_value(store::BACKEND_HOST.clone())
            .unwrap()
            .as_str(),
        store::get_settings_value(store::BACKEND_PORT.clone())
            .unwrap()
            .as_str()
    ))?;
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

pub use ifaces::sdk_info_iface::SdkInfoIfaceSyncClient;
impl ThriftDefaultClient
    for ifaces::sdk_info_iface::SdkInfoIfaceSyncClient<ClientInputProtocol, ClientOutputProtocol>
{
    fn default_client() -> Result<Self>
    where
        Self: Sized,
    {
        let (i_prot, o_prot) = get_io_protocols("sdkInfoIface")?;
        Ok(ifaces::sdk_info_iface::SdkInfoIfaceSyncClient::new(
            i_prot, o_prot,
        ))
    }
}

pub use ifaces::sdk_thrift_iface::SdkThriftIfaceSyncClient;
impl ThriftDefaultClient
    for ifaces::sdk_thrift_iface::SdkThriftIfaceSyncClient<
        ClientInputProtocol,
        ClientOutputProtocol,
    >
{
    fn default_client() -> Result<Self>
    where
        Self: Sized,
    {
        let (i_prot, o_prot) = get_io_protocols("sdkThriftIface")?;
        Ok(ifaces::sdk_thrift_iface::SdkThriftIfaceSyncClient::new(
            i_prot, o_prot,
        ))
    }
}

pub use ifaces::sdk_gen_code_iface::SdkGenCodeIfaceSyncClient;
impl ThriftDefaultClient
    for ifaces::sdk_gen_code_iface::SdkGenCodeIfaceSyncClient<
        ClientInputProtocol,
        ClientOutputProtocol,
    >
{
    fn default_client() -> Result<Self> {
        let (i_prot, o_prot) = get_io_protocols("sdkGenCodeIface")?;
        Ok(ifaces::sdk_gen_code_iface::SdkGenCodeIfaceSyncClient::new(
            i_prot, o_prot,
        ))
    }
}

#[cfg(test)]
mod thrift_test {
    use super::super::gen::ifaces;
    use crate::core::error::*;
    use crate::thrift::client::ThriftDefaultClient;
    use crate::thrift::gen::ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient;

    #[test]
    pub fn default_client() -> Result<()> {
        ifaces::sdk_gen_code_iface::SdkGenCodeIfaceSyncClient::default_client()?;
        Ok(())
    }
    #[test]
    pub fn test_client() -> Result<()> {
        let mut client = ifaces::sdk_thrift_iface::SdkThriftIfaceSyncClient::default_client()?;
        let result = client.get_executable_file_path();
        assert!(result.is_ok());
        Ok(())
    }
}
