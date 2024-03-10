use crate::core::error::Error;
use pest::Parser;
use pest_derive::Parser;

pub fn check_restful_code_err(content: &str) -> super::CheckResult {
    let result = RestfulParser::parse(Rule::root, content);
    if result.is_ok() {
        return super::CheckResult::default();
    }
    let err = result.err().unwrap();
    eprintln!("{:?}", &err);
    super::CheckResult::from(err)
}

pub fn parse_json_from_string(content: String) -> Result<serde_json::Value, Error> {
    let pairs = RestfulParser::parse(Rule::root, content.as_str())?;
    let first = pairs.into_iter().next();
    if first.is_none() {
        return Err("import restful_dsl failed. missing root!".into());
    }
    let root = first.unwrap().into_inner();
    Ok(serde_json::json!(&root))
}

#[derive(Parser)]
#[grammar = "_pest_home/restful.pest"]
pub struct RestfulParser;

#[cfg(test)]
mod restful_test {
    use super::{RestfulParser, Rule};
    use pest::Parser;

    #[test]
    fn err() {
        let result = super::check_restful_code_err(
            r#"
                namespace java org.java
                import "../"
                asd
                "#,
        );
        eprint!("{:?}", result);
    }

    #[test]
    fn namespace() {
        let result =
            RestfulParser::parse(Rule::namespace, "namespace java   as_d.asd.asdddasd_GFA");
        assert!(result.is_ok());
        let result = RestfulParser::parse(Rule::namespace, "namespace java _asd");
        assert!(result.is_ok());
        let result = RestfulParser::parse(Rule::namespace, "namespace ts com.org.tools");
        assert!(result.is_ok());
    }

    #[test]
    fn import() {
        let result = RestfulParser::parse(Rule::import, r#"import  "../test.restl""#);
        assert!(result.is_ok());
        let result = RestfulParser::parse(Rule::import, r#"import  "D:\test.restl""#);
        assert!(result.is_err());
    }

    #[test]
    fn r#type() {
        let result = RestfulParser::parse(Rule::r#type, "UserEntity");
        assert!(result.is_ok());
        let result = RestfulParser::parse(Rule::r#type, "Namespace.UserEntity");
        assert!(result.is_ok());
        let result = RestfulParser::parse(Rule::r#type, "Namespace.User1_Entity");
        assert!(result.is_ok());
    }

    #[test]
    fn class_field() {
        let result =
            RestfulParser::parse(Rule::class_field, "map<i64, map<double,list<string>>>attr");
        assert!(result.is_ok());
        let result = RestfulParser::parse(Rule::class_field, "optional Users.UserInfo attr");
        assert!(result.is_ok());
    }

    #[test]
    fn r#enum() {
        let result = RestfulParser::parse(
            Rule::r#enum,
            r#"enum Platform{
                    ANDROID =1,
                    WINDOWS =2
                }"#,
        );
        assert!(result.is_ok());
        let result = RestfulParser::parse(
            Rule::r#enum,
            r#"enum Platform {
                }"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn interface() {
        let result = RestfulParser::parse(
            Rule::interface,
            r#"interface Asd{
                    i64 getNumber(i64 id, required list<string> name)
                }"#,
        );
        assert!(result.is_ok());
        let result = RestfulParser::parse(
            Rule::interface,
            r#"interface Asd {
                    System.User getUser(optional map<i64, Attributes> data)
                }"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn root() {
        let content = r#" namespace java com.github.alphafoxz.oneboot.sdk.gen.restl.ifaces
            /*测试用。。。。格式不要太标准

            */
            namespace ts sdk.restl
            import "../structs/SdkResponseStruct.restl"

            //rest类型
            enum ThriftTypeEnum {
                SERVER=1,
                CLIENT = 2
            }

            class ThriftUpdateParam {
                //主键
                required i64 id
            string name
            ThriftTypeEnum type
                optional map<string, string>info
                optional SdkResponseStruct.SdkStringResponseStruct ext
            }

            class ThriftUpdateResponse
            {
                required boolean success
            }

            interface SdkThriftIface {
                ThriftUpdateResponse testFunction (optional ThriftUpdateParam  param )
            } "#;
        let result = RestfulParser::parse(Rule::root, content);
        assert!(result.is_ok());
    }
}
