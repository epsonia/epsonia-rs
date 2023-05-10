use std::path::Path;

#[derive(PartialEq, Clone)]
pub struct Check {
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub completed: bool,
    pub kind: CheckKind,
}

#[derive(PartialEq, Clone)]
pub enum CheckKind {
    FileExists {
        file_path: String,
        should_exist: bool,
    },
}

impl Check {
    pub fn new(
        points: i32,
        message: String,
        penalty_message: String,
        completed: bool,
        kind: CheckKind,
    ) -> Self {
        Check {
            points,
            message,
            penalty_message,
            completed,
            kind,
        }
    }
    pub fn run_check(&mut self) -> Self {
        self.completed = match &self.kind {
            CheckKind::FileExists {
                file_path,
                should_exist,
            } => Path::new(file_path).exists() == *should_exist,
        };

        self.clone()
    }
}
