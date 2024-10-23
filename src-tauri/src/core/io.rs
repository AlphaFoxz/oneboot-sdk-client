use crate::core::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    is_file: bool,
    is_folder: bool,
    path: String,
    content: Option<String>,
}

#[tauri::command]
pub async fn write_code_files(files: Vec<FileInfo>) -> Result<(), Error> {
    for file in files {
        let parent_dir = std::path::Path::new(&file.path).parent().unwrap();
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
        if file.is_file {
            fs::write(&file.path, file.content.unwrap())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn read_folder_content(
    parent_folder: String,
    folder_path_pattern: Option<String>,
    file_path_pattern: Option<String>,
) -> Result<Vec<FileInfo>, Error> {
    let paths = fs::read_dir(parent_folder)?;

    let mut files: Vec<FileInfo> = Vec::new();
    let match_folder = folder_path_pattern.is_some();
    let folder_path_pattern =
        regex::Regex::new(folder_path_pattern.unwrap_or(".*".to_string()).as_str()).unwrap();
    let match_file = file_path_pattern.is_some();
    let file_path_pattern =
        regex::Regex::new(file_path_pattern.unwrap_or(".*".to_string()).as_str()).unwrap();
    for path in paths {
        let path = path?.path().to_string_lossy().to_string();
        // 判断路径是否匹配正则
        let is_file = fs::metadata(&path)?.is_file();
        let is_folder = fs::metadata(&path)?.is_dir();
        if is_file && match_file && !file_path_pattern.is_match(path.as_str()) {
            continue;
        } else if is_folder && match_folder && !folder_path_pattern.is_match(path.as_str()) {
            continue;
        }
        files.push(FileInfo {
            is_file,
            is_folder,
            path: path.clone(),
            content: match is_file {
                true => Some(fs::read_to_string(&path)?),
                false => None,
            },
        });
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_match() {
        let path = "F:\\idea_projects\\oneboot\\domain-preset_sys\\src\\main\\java\\com\\github\\alphafoxz\\oneboot\\domain\\preset_sys\\user\\vo\\AccountVo.java";
        let pattern = "^F:\\\\idea_projects\\\\oneboot\\\\domain-preset_sys\\\\src\\\\main\\\\java\\\\com\\\\github\\\\alphafoxz\\\\oneboot\\\\domain\\\\preset_sys\\\\([a-zA-Z0-9_]+)\\\\vo\\\\.*[.]java$".to_string();
        assert!(path.matches(&pattern).count() > 0);
    }

    #[test]
    fn test_match2() {
        let path = "F:\\idea_projects";
        let pattern = "^F:\\idea_projects$".to_string();
        let pattern = Regex::new(pattern.replace("\\", "\\\\").as_str()).unwrap();
        assert!(pattern.is_match(path));
    }
}
