use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileExists {
    pub file_path: String,
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub should_exist: bool,
}
