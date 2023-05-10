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
    FileLineContains {
        file_path: String,
        line: i32,
        line_content: String,
        should_contain: bool,
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
            CheckKind::FileLineContains {
                file_path,
                line,
                line_content,
                should_contain,
            } => {
                let file = std::fs::read_to_string(file_path).unwrap();
                let lines: Vec<&str> = file.split('\n').collect();
                let line = lines[(*line - 1) as usize];
                line.contains(line_content) == *should_contain
            }
        };

        self.clone()
    }
}
