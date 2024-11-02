pub mod restful;

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

pub fn parse_restful_root_into_json(content: &String) -> Result<serde_json::Value, Error> {
    let pairs = restful::RestfulParser::parse(restful::Rule::root, content.as_str())?;
    let first = pairs.into_iter().next();
    if first.is_none() {
        return Err("import bmpn failed. missing root!".into());
    }
    let root = first.unwrap().into_inner();
    Ok(serde_json::json!(&root))
}
