use super::check::{Check, CheckKind};
use std::path::Path;

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

impl Check for FileExists {
    fn run_check(&mut self) {
        self.completed = Path::new(self.file_path.as_str()).exists();
    }

    fn kind(&self) -> CheckKind {
        CheckKind::FileExists
    }
}
