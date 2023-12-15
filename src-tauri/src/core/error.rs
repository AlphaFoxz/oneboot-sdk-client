use error_chain::error_chain;
use std::{error, fmt};

#[derive(Debug)]
pub enum OnebootError {
    Websocket(String),
}

impl fmt::Display for OnebootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for OnebootError {}

error_chain! {
    foreign_links {
        IoError(std::io::Error);
        OnebootError(OnebootError);
        ThriftError(thrift::Error);
        PestParseThriftError(pest::error::Error<super::parser::thrift::Rule>);
        PestParseRestError(pest::error::Error<super::parser::restful::Rule>);
        SerdeJsonError(serde_json::Error);
        InfallibleError(std::convert::Infallible);
    }
}
