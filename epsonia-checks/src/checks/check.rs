use core::fmt;

#[derive(PartialEq)]
pub enum CheckKind {
    FileExists,
    UserHasToExist,
}

pub trait Check {
    fn run_check(&mut self);

    fn kind(&self) -> CheckKind;

    // Completed
    fn is_completed(&self) -> bool;

    fn set_is_completed(&mut self, is_completed: bool);

    // Score amount
    fn score(&self) -> i32;

    // Score message
    fn message(&self) -> String;

    // Penalty message
    fn penalty_message(&self) -> String;
}

impl PartialEq for dyn Check {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}
