use epsonia_checks::check::{Check, CheckKind};
use serde::{Deserialize, Serialize};

use crate::models::FileExists;

// Note: Completed is a config value.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChecksConfig {
    pub file_exists_check: Vec<FileExists>,
}

pub fn parse_checks_config() -> ChecksConfig {
    let config = std::fs::read_to_string("./config/checks.json").unwrap();
    serde_json::from_str(&config).unwrap()
}

// Run before engine
pub fn get_max_points(checks: &Vec<Check>) -> i32 {
    let mut max_points = 0;

    for check in checks {
        max_points += check.points;
    }

    max_points
}

pub fn get_checks() -> Vec<Check> {
    let checks_config = parse_checks_config();

    let mut checks: Vec<Check> = Vec::new();

    for check in checks_config.file_exists_check {
        checks.push(Check::new(
            check.points,
            check.message,
            check.penalty_message,
            false,
            CheckKind::FileExists {
                file_path: check.file_path,
            },
        ));
    }

    checks
}
