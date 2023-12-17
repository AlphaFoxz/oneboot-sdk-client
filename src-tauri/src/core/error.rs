use std::fmt::Display;

#[derive(Debug)]
pub struct OnebootError {
    msg: String,
}
impl Display for OnebootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for OnebootError {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    OnebootError(OnebootError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ThriftError(#[from] thrift::Error),
    #[error(transparent)]
    PestParseThriftError(#[from] pest::error::Error<super::parser::thrift::Rule>),
    #[error(transparent)]
    PestParseRestError(#[from] pest::error::Error<super::parser::restful::Rule>),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    InfallibleError(#[from] std::convert::Infallible),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Into<Error> for &str {
    fn into(self) -> Error {
        Error::OnebootError(OnebootError {
            msg: self.to_string(),
        })
    }
}
