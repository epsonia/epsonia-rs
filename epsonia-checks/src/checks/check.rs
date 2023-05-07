use core::fmt;

#[derive(PartialEq)]
pub enum CheckKind {
    FileExists,
    UserHasToExist,
}

pub trait Check {
    fn run_check(&mut self);

    fn kind(&self) -> CheckKind;
}
