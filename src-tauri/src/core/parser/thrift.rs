use crate::core::error::Error;
use pest::Parser;
use pest_derive::Parser;

pub fn check_thrift_code_err(content: &str) -> super::CheckResult {
    let result = ThriftParser::parse(Rule::root, content);
    if result.is_ok() {
        return super::CheckResult::default();
    }
    let err = result.err().unwrap();
    eprintln!("{:?}", &err);
    super::CheckResult::from(err)
}

pub fn parse_json_from_string(content: String) -> Result<serde_json::Value, Error> {
    let pairs = ThriftParser::parse(Rule::root, content.as_str())?;
    let first = pairs.into_iter().next();
    if first.is_none() {
        return Err("import bmpn failed. missing root!".into());
    }
    let root = first.unwrap().into_inner();
    Ok(serde_json::json!(&root))
}

#[derive(Parser)]
#[grammar = "_pest_home/thrift.pest"]
pub struct ThriftParser;

#[cfg(test)]
mod thrift_test {
    use super::{Rule, ThriftParser};
    use pest::Parser;

    #[test]
    fn err() {
        let result = super::check_thrift_code_err(
            r#"
                namespace java org.java
                include "../"
                asd
                "#,
        );
        eprint!("{:?}", result);
    }

    #[test]
    fn namespace() {
        let result = ThriftParser::parse(Rule::namespace, "namespace java   as_d.asd.asdddasd_GFA");
        assert!(result.is_ok());
        let result = ThriftParser::parse(Rule::namespace, "namespace java _asd");
        assert!(result.is_ok());
        let result = ThriftParser::parse(Rule::namespace, "namespace js com.org.tools");
        assert!(result.is_ok());
    }

    #[test]
    fn include() {
        let result = ThriftParser::parse(Rule::include, r#"include  "../test.thrift""#);
        assert!(result.is_ok());
        let result = ThriftParser::parse(Rule::include, r#"include  "D:\test.thrift""#);
        assert!(result.is_err());
    }

    #[test]
    fn r#type() {
        let result = ThriftParser::parse(Rule::r#type, "UserEntity");
        assert!(result.is_ok());
        let result = ThriftParser::parse(Rule::r#type, "Namespace.UserEntity");
        assert!(result.is_ok());
        let result = ThriftParser::parse(Rule::r#type, "Namespace.User1_Entity");
        assert!(result.is_ok());
    }

    #[test]
    fn struct_attribute() {
        let result = ThriftParser::parse(
            Rule::struct_attribute,
            "1:required map<i64, map<double,list<string>>>attr",
        );
        assert!(result.is_ok());
        let result = ThriftParser::parse(Rule::struct_attribute, "2:optional Users.UserInfo attr");
        assert!(result.is_ok());
    }

    #[test]
    fn r#enum() {
        let result = ThriftParser::parse(
            Rule::r#enum,
            r#"enum Platform{
                    ANDROID =1,
                    WINDOWS
                }"#,
        );
        assert!(result.is_ok());
        let result = ThriftParser::parse(
            Rule::r#enum,
            r#"enum Platform {
                }"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn service() {
        let result = ThriftParser::parse(
            Rule::service,
            r#"service Asd{
                    i64 getNumber(1:i64 id, 2: required list<string> name)
                }"#,
        );
        assert!(result.is_ok());
        let result = ThriftParser::parse(
            Rule::service,
            r#"service Asd {
                    System.User getUser(1:optional map<i64, Attributes> data)
                }"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn root() {
        let content = r#" namespace java com.github.alphafoxz.oneboot.sdk.gen.thrift.ifaces
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
                ThriftUpdateResponse testFunction (1: optional ThriftUpdateParam  param )
            } "#;
        let result = ThriftParser::parse(Rule::root, content);
        assert!(result.is_ok());
    }
}
