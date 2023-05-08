use serde::{Deserialize, Serialize};

use super::check::CheckData;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileExists {
    file_path: String,
    points: i32,
    message: String,
    penalty_message: String,
    completed: bool,
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
        self.set_is_completed(Path::new(&self.file_path).exists());
    }

    fn is_completed(&self) -> &bool {
        &self.completed
    }

    fn set_is_completed(&mut self, is_completed: bool) {
        self.completed = is_completed;
    }

    fn points(&self) -> &i32 {
        &self.points
    }

    fn set_points(&mut self, points: i32) {
        self.points = points;
    }

    fn message(&self) -> &String {
        &self.message
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }

    fn penalty_message(&self) -> &String {
        &self.penalty_message
    }

    fn set_penalty_message(&mut self, penalty_message: String) {
        self.penalty_message = penalty_message;
    }
}
