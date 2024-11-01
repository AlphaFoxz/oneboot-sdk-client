use crate::core::error::Error;
use serde::{Deserialize, Serialize};
use strsim::{
    jaro, jaro_winkler, normalized_damerau_levenshtein, normalized_levenshtein, sorensen_dice,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTarget {
    name: String,
    score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResult {
    pub name: String,
    pub selected: Option<String>,
    pub targets: Vec<MatchTarget>,
}

#[tauri::command]
pub fn match_similar_strings(
    sources: Vec<String>,
    targets: Vec<String>,
) -> Result<Vec<MatchResult>, Error> {
    let mut result = vec![];
    for source in sources {
        let mut match_result = vec![];
        for target in targets.clone() {
            let score = match_field(source.clone(), target.clone());
            match_result.push(MatchTarget {
                name: target.clone(),
                score: score.unwrap_or(0.0),
            });
        }
        match_result.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        result.push(MatchResult {
            name: source,
            selected: None,
            targets: match_result,
        });
    }
    Ok(result)
}

fn match_field(source: String, target: String) -> Option<f64> {
    let binding = split_camel_with_space(&source);
    let d_source = binding.as_str();
    let binding = split_camel_with_space(&target);
    let d_target = binding.as_str();
    let scores = vec![
        jaro(d_source, d_target),
        jaro_winkler(d_source, d_target),
        normalized_levenshtein(d_source, d_target),
        normalized_damerau_levenshtein(d_source, d_target),
        sorensen_dice(d_source, d_target),
    ];
    max(scores)
}

// 将驼峰变量名用空格分隔
fn split_camel_with_space(name: &String) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push(' ');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

fn max(arr: Vec<f64>) -> Option<f64> {
    arr.iter().fold(None, |max, x| {
        if max.is_none() {
            return Some(*x);
        } else if max.unwrap() < *x {
            return Some(*x);
        }
        return None;
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_table_name() {
        let source = "UserVo.java".to_string();
        let target = "PsysUser.java".to_string();
        let score = match_field(source, target);
        assert!(score.unwrap() > 0.5);

        let source = "TokenVo.java".to_string();
        let target = "PsysToken.java".to_string();
        let score = match_field(source, target);
        assert!(score.unwrap() > 0.5);

        let source = "AccountVo.java".to_string();
        let target = "PsysAccount.java".to_string();
        let score = match_field(source, target);
        assert!(score.unwrap() > 0.5);
    }
}
