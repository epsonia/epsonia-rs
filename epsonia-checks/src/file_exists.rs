use serde::{Deserialize, Serialize};

use super::check::CheckData;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileExists {
    pub file_path: String,
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub completed: bool,
}

impl FileExists {
    pub fn new(file_path: String, points: i32, message: String, penalty_message: String) -> Self {
        FileExists {
            file_path,
            points,
            message,
            penalty_message,
            // Note to self: Later, add a param of penalty which controls this.
            // just a stupid theory in my head:
            completed: false,
        }
    }
}

impl CheckData for FileExists {
    fn run_check(&mut self) {
        self.completed = Path::new(&self.file_path).exists();
    }
}
