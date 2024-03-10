pub mod domain;
pub mod restful;
pub mod thrift;

use crate::core::error::Error;
use pest::Parser;

#[derive(Debug, serde::Serialize)]
pub struct CheckResult {
    pub success: bool,
    pub location: Option<ErrorLocation>,
    pub line_col: Option<ErrorLocation>,
    pub message: Option<String>,
}
impl Default for CheckResult {
    fn default() -> Self {
        Self {
            success: true,
            location: None,
            line_col: None,
            message: None,
        }
    }
}
impl From<pest::error::Error<thrift::Rule>> for CheckResult {
    fn from(value: pest::error::Error<thrift::Rule>) -> Self {
        let mut check_result = CheckResult::default();
        check_result.success = false;
        check_result.location = Some(ErrorLocation::from(value.location));
        check_result.line_col = Some(ErrorLocation::from(value.line_col));
        match value.variant {
            pest::error::ErrorVariant::ParsingError {
                positives,
                negatives,
            } => {
                check_result.message = Some(format!(
                    "期待的元素：{:?}，意外的元素：{:?}",
                    positives, negatives
                ));
            }
            pest::error::ErrorVariant::CustomError { message } => {
                check_result.message = Some(message)
            }
        }
        check_result
    }
}
impl From<pest::error::Error<restful::Rule>> for CheckResult {
    fn from(value: pest::error::Error<restful::Rule>) -> Self {
        let mut check_result = CheckResult::default();
        check_result.success = false;
        check_result.location = Some(ErrorLocation::from(value.location));
        check_result.line_col = Some(ErrorLocation::from(value.line_col));
        match value.variant {
            pest::error::ErrorVariant::ParsingError {
                positives,
                negatives,
            } => {
                check_result.message = Some(format!(
                    "期待的元素：{:?}，意外的元素：{:?}",
                    positives, negatives
                ));
            }
            pest::error::ErrorVariant::CustomError { message } => {
                check_result.message = Some(message)
            }
        }
        check_result
    }
}
impl From<pest::error::Error<domain::Rule>> for CheckResult {
    fn from(value: pest::error::Error<domain::Rule>) -> Self {
        let mut check_result = CheckResult::default();
        check_result.success = false;
        check_result.location = Some(ErrorLocation::from(value.location));
        check_result.line_col = Some(ErrorLocation::from(value.line_col));
        match value.variant {
            pest::error::ErrorVariant::ParsingError {
                positives,
                negatives,
            } => {
                check_result.message = Some(format!(
                    "期待的元素：{:?}，意外的元素：{:?}",
                    positives, negatives
                ));
            }
            pest::error::ErrorVariant::CustomError { message } => {
                check_result.message = Some(message)
            }
        }
        check_result
    }
}

#[derive(Debug, serde::Serialize)]
pub enum ErrorLocation {
    Pos(usize),
    PosLine((usize, usize)),
    Span((usize, usize)),
    SpanLine((usize, usize), (usize, usize)),
}
impl From<pest::error::InputLocation> for ErrorLocation {
    fn from(value: pest::error::InputLocation) -> Self {
        match value {
            pest::error::InputLocation::Pos(pos) => {
                return ErrorLocation::Pos(pos);
            }
            pest::error::InputLocation::Span(pos) => {
                return ErrorLocation::Span(pos);
            }
        }
    }
}
impl From<pest::error::LineColLocation> for ErrorLocation {
    fn from(value: pest::error::LineColLocation) -> Self {
        match value {
            pest::error::LineColLocation::Pos(pos) => {
                return ErrorLocation::PosLine(pos);
            }
            pest::error::LineColLocation::Span(start, end) => {
                return ErrorLocation::SpanLine(start, end);
            }
        }
    }
}

pub fn parse_thrift_root_into_json(content: &String) -> Result<serde_json::Value, Error> {
    let pairs = thrift::ThriftParser::parse(thrift::Rule::root, content.as_str())?;
    let first = pairs.into_iter().next();
    if first.is_none() {
        return Err("import bmpn failed. missing root!".into());
    }
    let root = first.unwrap().into_inner();
    Ok(serde_json::json!(&root))
}

pub fn parse_restful_root_into_json(content: &String) -> Result<serde_json::Value, Error> {
    let pairs = restful::RestfulParser::parse(restful::Rule::root, content.as_str())?;
    let first = pairs.into_iter().next();
    if first.is_none() {
        return Err("import bmpn failed. missing root!".into());
    }
    let root = first.unwrap().into_inner();
    Ok(serde_json::json!(&root))
}

#[cfg(test)]
mod parser_test {
    #[test]
    fn parse_json() {
        let content = String::from(
            r#"
            namespace java com.github.alphafoxz.oneboot.sdk.gen.thrift.ifaces
            namespace rs thrift.ifaces
            include "../structs/SdkResponseStruct.thrift"
            
            service SdkThriftIface {
                /*创建全部rpc代码*/
                SdkResponseStruct.SdkListResponseStruct generateAll()
                /*获取当前rpc服务端口*/
                SdkResponseStruct.SdkLongResponseStruct getServerPort()
                /*获取thrift可执行文件路径*/
                SdkResponseStruct.SdkStringResponseStruct getExecutableFilePath()
            }"#,
        );
        let result = super::parse_thrift_root_into_json(&content);
        assert!(result.is_ok());
        let content = String::from("namespace java com.github.alphafoxz.oneboot.sdk.gen.thrift.ifaces\r\nnamespace rs thrift.ifaces\r\ninclude \"../dtos/SdkRequestDto.thrift\"\r\ninclude \"../dtos/SdkResponseDto.thrift\"\r\n\r\nservice SdkGenCodeIface {\r\n    /*创建全部Rpc代码*/\r\n    SdkResponseDto.SdkListResponseDto generateJavaRpc(1:required i64 taskId)\r\n    /*创建java的Api代码*/\r\n    SdkResponseDto.SdkListResponseDto generateJavaApi(1:required SdkRequestDto.SdkStringRequestDto jsonDto, 2:required map<string, SdkRequestDto.SdkStringRequestDto> importJsonDto)\r\n    /*创建java的dto代码*/\r\n    SdkResponseDto.SdkListResponseDto generateJavaDto(1:required SdkRequestDto.SdkStringRequestDto jsonDto)\r\n}");
        let result = super::parse_thrift_root_into_json(&content);
        assert!(result.is_ok());
        eprintln!("{}", result.unwrap());
        let content = String::from(
            r#" namespace java com.github.alphafoxz.oneboot.sdk.gen.thrift.ifaces
            /*测试用。。。。格式不要太标准

            */
            namespace rs thrift.ifaces
            include "../structs/SdkResponseStruct.thrift"

            //Thrift类型
            enum ThriftTypeEnum {
                SERVER,
                CLIENT = 2
            }

            struct ThriftUpdateParam {
                //主键
                1:required i64 id
                2:string name
                3:ThriftTypeEnum type
                4:optional map<string, string>info
                5:optional SdkResponseStruct.SdkStringResponseStruct ext
            }

            struct ThriftUpdateResponse
            {
                1:required bool success
            }

            service SdkThriftIface {
                ThriftUpdateResponse testFunction (1:  ThriftUpdateParam  param )
            } "#,
        );
        let result = super::parse_thrift_root_into_json(&content);
        assert!(result.is_ok());
    }
}
