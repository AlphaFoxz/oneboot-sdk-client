use crate::core::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "_pest_home/domain.pest"]
pub struct DomainParser;

pub fn check_domain_code_err(content: &str) -> super::CheckResult {
    let result = DomainParser::parse(Rule::root, content);
    if result.is_ok() {
        return super::CheckResult::default();
    }
    let err = result.err().unwrap();
    eprintln!("{:?}", &err);
    super::CheckResult::from(err)
}

pub fn parse_json_from_string(content: String) -> Result<serde_json::Value, Error> {
    let pairs = DomainParser::parse(Rule::root, content.as_str())?;
    let first = pairs.into_iter().next();
    if first.is_none() {
        return Err("import domain_dsl failed. missing root!".into());
    }
    let root = first.unwrap().into_inner();
    Ok(serde_json::json!(&root))
}

#[cfg(test)]
mod domain_test {
    use super::{DomainParser, Rule};
    use pest::Parser;

    #[test]
    fn element_test() {
        let result = DomainParser::parse(Rule::element_name_value, "TestEvent");
        assert!(result.is_ok());
        let result = DomainParser::parse(Rule::element_name, "(TestEvent)");
        assert!(result.is_err());
        let result = DomainParser::parse(Rule::element_name, "( \"TestEvent\")");
        assert!(result.is_ok())
    }
    #[test]
    fn root() {
        let result = DomainParser::parse(
            Rule::root,
            r#"@startuml
'事件
DomainEvent("Sd") [
'测试
  --rule--
  1. 规则
  ~=.。[+-*/\_?？:：<>《》",，%$￥#@!！|
--field--
  '123
  string i32
]
@enduml"#,
        );
        assert!(result.is_ok())
    }
}
