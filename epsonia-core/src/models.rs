use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileExists {
    pub file_path: String,
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub should_exist: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLineContains {
    pub file_path: String,
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub line: i32,
    pub line_content: String,
    pub should_contain: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileContainsContent {
    pub file_path: String,
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub content: String,
    pub whitespace_matters: bool,
    pub should_contain: bool,
}
