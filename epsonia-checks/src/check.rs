use super::file_exists::FileExists;

#[derive(PartialEq)]
pub enum CheckKind {
    FileExists,
    UserHasToExist,
}

#[derive(PartialEq, Clone)]
pub enum Checks {
    FileExists(FileExists),
}

impl CheckData for Checks {
    fn run_check(&mut self) {
        match self {
            Checks::FileExists(c) => c.run_check(),
            _ => panic!("This check doesn't have a run_check field."),
        }
    }

    fn is_completed(&self) -> &bool {
        match self {
            Checks::FileExists(c) => c.is_completed(),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn set_is_completed(&mut self, is_completed: bool) {
        match self {
            Checks::FileExists(c) => c.set_is_completed(is_completed),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn points(&self) -> &i32 {
        match self {
            Checks::FileExists(c) => c.points(),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn set_points(&mut self, points: i32) {
        match self {
            Checks::FileExists(c) => c.set_points(points),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn message(&self) -> &String {
        match self {
            Checks::FileExists(c) => c.message(),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn set_message(&mut self, message: String) {
        match self {
            Checks::FileExists(c) => c.set_message(message),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn penalty_message(&self) -> &String {
        match self {
            Checks::FileExists(c) => c.penalty_message(),
            _ => panic!("This check doesn't have a completed field."),
        }
    }

    fn set_penalty_message(&mut self, penalty_message: String) {
        match self {
            Checks::FileExists(c) => c.set_penalty_message(penalty_message),
            _ => panic!("This check doesn't have a completed field."),
        }
    }
}

pub trait CheckData {
    fn run_check(&mut self);

    // Completed
    fn is_completed(&self) -> &bool;

    fn set_is_completed(&mut self, is_completed: bool);

    // Score amount
    fn points(&self) -> &i32;

    fn set_points(&mut self, points: i32);

    // Score message
    fn message(&self) -> &String;

    fn set_message(&mut self, message: String);

    // Penalty message
    fn penalty_message(&self) -> &String;

    fn set_penalty_message(&mut self, penalty_message: String);
}
