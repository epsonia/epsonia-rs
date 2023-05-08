use super::file_exists::FileExists;

#[derive(PartialEq, Clone)]
pub enum Checks {
    FileExists(FileExists),
}
