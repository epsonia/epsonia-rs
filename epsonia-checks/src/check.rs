use super::file_exists::FileExists;

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
}

pub trait CheckData {
    fn run_check(&mut self);
}
